use glium::backend::glutin::headless::Headless;
use glium::*;
use once_cell::sync::Lazy;
use std::marker::*;
use std::path::Path;
use std::sync::mpsc::*;
use std::thread::*;

mod monster_hitzone;

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
    pub fn save_png(&self, output: &Path) -> anyhow::Result<()> {
        let output = std::fs::File::create(output)?;
        let mut encoder = png::Encoder::new(output, self.width, self.height);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)?;
        Ok(())
    }
}
