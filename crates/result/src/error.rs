use std::fmt::Display;

#[derive(Debug)]
pub struct Backtrace {
    message: Option<String>,
    file: String,
    line: u32,
    column: u32,
}

impl Display for Backtrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match &self.message {
            Some(m) => m,
            None => "propagated..."
        };
        write!(f, "{} (at {}:{}:{})", msg, self.file, self.line, self.column)
    }
}

#[derive(Debug)]
pub struct Error {
    message: Option<String>,
    backtrace: Vec<Backtrace>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut message = self.backtrace.iter()
            .find(|e| e.message.as_ref().is_some())
            .map(|b| b.to_string()).unwrap_or_default();
        if self.backtrace.len() > 1 {
            eprintln!("Caused by:");
            for e in self.backtrace.iter().rev() {
                if !message.is_empty() { message += "\n" }
                message += &format!(" - {}", e);
            }
        }
        write!(f, "{}", message)
    }
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self { message: Some(message.into()), backtrace: vec![] }
    }

    pub fn propagate() -> Self {
        Self { message: None, backtrace: vec![] }
    }

    pub fn print(&self) {
        eprintln!("ERROR: {}", self.to_string());
    }

    pub fn backtrace(&self) -> &Vec<Backtrace> {
        &self.backtrace
    }
}

macro_rules! error_from {
    ($($t:ty),+) => {
        $(
            impl From<$t> for Error {
                fn from(value: $t) -> Self {
                    Self::new(&format!("{value}"))
                }
            }
        )+
    };
}

error_from! { &str, String, std::io::Error, std::num::ParseIntError }

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Self::propagate()
    }
}

pub trait ResultExt<T> {
    fn trace(self) -> crate::Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where E: std::error::Error {
    #[track_caller]
    fn trace(self) -> crate::Result<T> {
        let caller = std::panic::Location::caller();
        self.map_err(|e| {
            let mut error = Error::propagate();
            error.backtrace.push(Backtrace {
                file: caller.file().into(),
                column:caller.column(),
                line:caller.line(),
                message: Some(format!("{e}"))
            });
            error
        })
    }
}

impl<T> ResultExt<T> for std::result::Result<T, Error>
{
    #[track_caller]
    fn trace(self) -> crate::Result<T> {
        let caller = std::panic::Location::caller();
        self.map_err(|mut e| {
            e.backtrace.push(Backtrace {
                file: caller.file().into(),
                column:caller.column(),
                line:caller.line(),
                message: e.message.clone()
            });
            e
        })
    }
}
