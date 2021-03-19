mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}




// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     pub fn summer(toast: &str) -> Breakfast {
//         Breakfast {
//             toast: String::from(toast),
//             seasonal_fruit: String::from("桃子")
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::summer("黑麥");

//     meal.toast = String::from("全麥");
//     println!("我要外帶{}", meal.toast);

//     // meal.seasonal_fruit = String::from("藍莓");
// }



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
