
fn main() {

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);


    let char_list = vec!['y', 'e', 'l', 'l', 'o'];
    let result = largest(&char_list);
    println!("The largest character is {}", result);
}

fn largest<T: PartialOrd + Copy>(list:&[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}