fn main() {
    // another_function(5);

    // let x = five();
    let x = plus_one(4);

    let y = {
        let x = 3;
        x + 1
    };

    println!("x = {}", x);
    println!("y = {}", y);
}

fn plus_one (x: i32) -> i32 {
    x + 1
}

// fn five() -> i32 {
//     5
// }


// fn another_function(x: i32) {
//     println!("x = {}", x);
// }
