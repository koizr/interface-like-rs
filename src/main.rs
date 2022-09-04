use dynamic_lib;
use logger::{NoopLogger, StdoutLogger};
use static_lib;

fn main() {
    let env = std::env::var("ENV").unwrap_or("PRODUCTION".to_string());

    if env == "DEVELOPMENT" {
        dynamic_lib::run(Box::new(StdoutLogger));
    } else {
        dynamic_lib::run(Box::new(NoopLogger));
    }

    if cfg!(feature = "log") {
        static_lib::run(StdoutLogger);
    } else {
        static_lib::run(NoopLogger);
    }
}
