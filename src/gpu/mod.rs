use anyhow::bail;
use glium::backend::glutin::headless::Headless;
use glium::*;
use nalgebra_glm::*;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use std::marker::*;
use std::path::Path;
use std::sync::mpsc::*;
use std::thread::*;

// mod astc;
mod ffi;
mod monster_hitzone;

pub use ffi::*;
pub use monster_hitzone::*;

struct Job {
    f: Box<dyn FnOnce(&GlHandle) + Send + 'static>,
}

fn gpu_thread(receiver: Receiver<Job>) {
    #[cfg(target_family = "unix")]
    use glutin::platform::unix::EventLoopExtUnix;
    #[cfg(target_family = "windows")]
    use glutin::platform::windows::EventLoopExtWindows;

    let event_loop: glutin::event_loop::EventLoop<()> =
        glutin::event_loop::EventLoop::new_any_thread();
    let cb = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core);
    let size = glutin::dpi::PhysicalSize {
        width: 800,
        height: 600,
    };
    let context = cb.build_headless(&event_loop, size).unwrap();
    // SAFETY: this is the only GL context in the entire program
    let context = unsafe { context.treat_as_current() };
    let display = glium::backend::glutin::headless::Headless::new(context).unwrap();

    let gl_handle = GlHandle { display };
    for job in receiver.iter() {
        (job.f)(&gl_handle);
    }
}

struct GlHandle {
    display: Headless,
}

struct Context {
    job_sender: SyncSender<Job>,
}

static CONTEXT: Lazy<Context> = Lazy::new(|| {
    let (job_sender, job_receiver) = sync_channel(0);
    spawn(move || gpu_thread(job_receiver));
    Context { job_sender }
});

impl Context {
    fn run<R: Send + 'static, F: FnOnce(&GlHandle) -> R + Send + 'static>(&self, f: F) -> R {
        let (result_sender, result_receiver) = channel();
        self.job_sender
            .send(Job {
                f: Box::new(move |gl_handle| result_sender.send(f(gl_handle)).unwrap()),
            })
            .unwrap();
        result_receiver.recv().unwrap()
    }
}

pub struct RgbaImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl RgbaImage {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> RgbaImage {
        if data.len() != usize::try_from(width * height * 4).unwrap() {
            panic!("Wrong size")
        }
        RgbaImage {
            data,
            width,
            height,
        }
    }

    pub fn save_png(&self, output: &Path) -> anyhow::Result<()> {
        let output = std::fs::File::create(output)?;
        let mut encoder = png::Encoder::new(output, self.width, self.height);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;
        Ok(())
    }

    pub fn sub_image_f(&self, p0: Vec2, p1: Vec2) -> anyhow::Result<RgbaImage> {
        let x0 = (p0.x * self.width as f32).round() as u32;
        let y0 = (p0.y * self.height as f32).round() as u32;
        let x1 = (p1.x * self.width as f32).round() as u32;
        let y1 = (p1.y * self.height as f32).round() as u32;
        self.sub_image(x0, y0, x1 - x0, y1 - y0)
    }

    pub fn sub_image(
        &self,
        start_x: u32,
        start_y: u32,
        width: u32,
        height: u32,
    ) -> anyhow::Result<RgbaImage> {
        if start_x + width > self.width || start_y + height > self.height {
            bail!("Location out of bound")
        }

        let mut data = vec![0; usize::try_from(width * height * 4)?];
        for x in 0..width {
            for y in 0..height {
                let src = usize::try_from(x + start_x + self.width * (y + start_y))? * 4;
                let dst = usize::try_from(x + y * width)? * 4;
                data[dst..dst + 4].copy_from_slice(&self.data[src..src + 4]);
            }
        }
        Ok(RgbaImage {
            width,
            height,
            data,
        })
    }
}
