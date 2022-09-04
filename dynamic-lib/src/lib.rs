use logger::Logger;

struct App {
    logger: Box<dyn Logger>,
}

impl App {
    pub fn new(logger: Box<dyn Logger>) -> Self {
        Self { logger }
    }

    pub fn run(mut self) {
        self.logger.write("run dynamic-lib");
    }
}

// トレイトオブジェクトをスマートポインタなどで包んで上げる必要がある
// （実行するまで dyn Logger 部分のサイズがわからないので）
pub fn run(logger: Box<dyn Logger>) {
    App::new(logger).run();
}
