use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("請猜測一個數字");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("秘密數字為: {}", secret_number);

    loop {
        println!("請輸入猜測的數字");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("讀取該行失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你的猜測數據: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("贏了!");
                break;
            }
        }
    }
}
