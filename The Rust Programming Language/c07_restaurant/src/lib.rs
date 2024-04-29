pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house;

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        // 私有函数，不能从外部调用
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
use front_of_house::hosting::add_to_waitlist;
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
pub fn eat_at_restaurant() {
    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
    // 在夏天点一份黑麦面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("wheat");
    // 更改我们需要的面包
    meal.toast = String::from("whole wheat");
    println!("I'd like {} toast with .", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
}
