pub struct LoggerRoot {
    prefix: Vec<String>,
    buffer: String,
    line_start: bool,
}

impl LoggerRoot {
    pub fn new() -> Self {
        LoggerRoot {
            prefix: vec![],
            buffer: String::new(),
            line_start: true,
        }
    }

    pub fn logger(&mut self) -> Logger {
        Logger { root: self }
    }

    pub fn finalize(self) -> String {
        self.buffer
    }
}

pub struct Logger<'a> {
    root: &'a mut LoggerRoot,
}

impl<'a> Logger<'a> {
    pub fn scope<'b>(self: &'b mut Logger<'a>, prefix: String) -> Logger<'b> {
        self.root.prefix.push(prefix);
        Logger { root: self.root }
    }
}

impl<'a> std::fmt::Write for &mut Logger<'a> {
    fn write_str(&mut self, mut s: &str) -> std::fmt::Result {
        while !s.is_empty() {
            if self.root.line_start {
                for prefix in &self.root.prefix {
                    eprint!("{prefix}.");
                    write!(&mut self.root.buffer, "{prefix}.")?;
                }
                eprint!("|");
                write!(&mut self.root.buffer, "|")?;
            }
            let seg = if let Some(li) = s.find('\n') {
                self.root.line_start = true;
                let (seg, rest) = s.split_at(li + 1);
                s = rest;
                seg
            } else {
                self.root.line_start = false;
                std::mem::replace(&mut s, "")
            };
            eprint!("{}", seg);
            write!(&mut self.root.buffer, "{}", seg)?;
        }
        Ok(())
    }
}

macro_rules! lscope {
    ($logger:ident, $($t:tt)*) => {
        #[allow(unused_variables, unused_mut)]
        let mut $logger = &mut $logger.scope(format!($($t)*));
    };
}
pub(crate) use lscope;

impl<'a> Drop for Logger<'a> {
    fn drop(&mut self) {
        self.root.prefix.pop();
    }
}
