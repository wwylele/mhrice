use anyhow::bail;
use glium::backend::glutin::headless::Headless;
use glium::*;
use glutin::Context;
use glutin::NotCurrent;
use nalgebra_glm::*;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use std::io::Write;
use std::marker::*;
use std::sync::mpsc::*;
use std::sync::Mutex;
use std::thread::*;

mod bc7;
mod monster_hitzone;

pub use bc7::*;
pub use monster_hitzone::*;

static INIT_CONTEXT: Lazy<Mutex<Option<Context<NotCurrent>>>> = Lazy::new(|| Mutex::new(None));

struct Job {
    f: Box<dyn FnOnce(&GlHandle) + Send + 'static>,
}

// Must be called from the main thread
pub fn gpu_init() {
    let event_loop: glutin::event_loop::EventLoop<()> = glutin::event_loop::EventLoop::new();
    let cb = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core);
    let size = glutin::dpi::PhysicalSize {
        width: 800,
        height: 600,
    };
    if let Ok(context) = cb.build_headless(&event_loop, size) {
        *INIT_CONTEXT.lock().unwrap() = Some(context);
    }
}

fn gpu_thread(receiver: Receiver<Job>) {
    let context = INIT_CONTEXT
        .lock()
        .unwrap()
        .take()
        .expect("Failed to create GL context");
    // SAFETY: this is the only GL context in the entire program
    let context = unsafe { context.make_current().unwrap() };
    let display = glium::backend::glutin::headless::Headless::new(context).unwrap();

    let gl_handle = GlHandle { display };
    for job in receiver.iter() {
        (job.f)(&gl_handle);
    }
}

struct GlHandle {
    display: Headless,
}

struct GpuContext {
    job_sender: SyncSender<Job>,
}

static CONTEXT: Lazy<GpuContext> = Lazy::new(|| {
    let (job_sender, job_receiver) = sync_channel(0);
    spawn(move || gpu_thread(job_receiver));
    GpuContext { job_sender }
});

impl GpuContext {
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel(&mut self, x: u32, y: u32) -> &mut [u8; 4] {
        let pos = usize::try_from(x + y * self.width).unwrap() * 4;
        (&mut self.data[pos..][..4]).try_into().unwrap()
    }

    pub fn save_png(&self, output: impl Write) -> anyhow::Result<()> {
        let mut encoder = png::Encoder::new(output, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
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

    pub fn gen_double_mask(mut self) -> (RgbaImage, RgbaImage) {
        let width = self.width;
        let height = self.height;
        let mut data = vec![0; usize::try_from(width * height * 4).unwrap()];
        for x in 0..width {
            for y in 0..height {
                let pos = usize::try_from(x + y * width).unwrap() * 4;
                data[pos + 3] = self.data[pos];
                self.data[pos] = 0;
                self.data[pos + 1] = 0;
                self.data[pos + 2] = 0;
            }
        }
        (
            RgbaImage {
                width,
                height,
                data,
            },
            self,
        )
    }
}
