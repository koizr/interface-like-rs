use dynamic_lib;
use logger::{NoopLogger, StdoutLogger};
use static_lib;

fn main() {
    // 動的ディスパッチを使う場合は、環境変数などランタイムに決定される値によって実装を切り替えることができる
    let env = std::env::var("ENV").unwrap_or("PRODUCTION".to_string());
    if env == "DEVELOPMENT" {
        dynamic_lib::run(Box::new(StdoutLogger));
    } else {
        dynamic_lib::run(Box::new(NoopLogger));
    }

    // 静的ディスパッチを使う場合は、 feature フラグなどコンパイルタイムに決定される値によって実装を切り替えることができる
    if cfg!(feature = "log") {
        static_lib::run(StdoutLogger);
    } else {
        static_lib::run(NoopLogger);
    }
}
