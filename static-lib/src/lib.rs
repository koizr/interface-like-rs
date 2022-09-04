use logger::Logger;

struct App<L: Logger> {
    logger: L,
}

impl<L: Logger> App<L> {
    pub fn new(logger: L) -> Self {
        Self { logger }
    }

    pub fn run(mut self) {
        self.logger.write("run static-lib");
    }
}

pub fn run<L: Logger>(logger: L) {
    App::new(logger).run();
}
