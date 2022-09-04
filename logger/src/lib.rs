pub trait Logger {
    fn write(&mut self, message: &str);
}

pub struct NoopLogger;

impl Logger for NoopLogger {
    fn write(&mut self, _: &str) {
        ()
    }
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
    fn write(&mut self, message: &str) {
        println!("{message}");
    }
}
