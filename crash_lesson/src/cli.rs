use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let commend  = args[1].clone();

    println!("commend: {:?}", commend);
}
