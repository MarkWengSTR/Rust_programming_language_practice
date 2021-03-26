use std::env;
use std::process;

use functionals::Config;
use functionals::run;


fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("解析引數時出現問題: {}", err);
        process::exit(1);
    });

    println!("搜尋 {}", config.query);
    println!("目標檔案 {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("應用程式錯誤: {}", e);

        process::exit(1);
    }
}
