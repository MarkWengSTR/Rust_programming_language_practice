use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
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
