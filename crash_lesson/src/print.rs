pub fn run() {
    // print to console
    println!("Hello hahaha");

    // Basic Formatting
    println!("Number: {}, {}", 1, 2);

    // Positional Arguments
    println!("{0} is from {1} and {0} like to {2}", "Brad", "Mas", "code");

    // Name Arguments
    println!("{name} like to play {activity}", name="John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 , 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}

