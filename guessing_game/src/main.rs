use std::io;
use rand::Rng;  // Rng 是一个trait，它定义了随机数生成器应实现的方法
use std::cmp::Ordering; // Ordering也是一个枚举，不过他的成员是Less, Greater, Equal。
// 为了获取用户输入并打印结果作为输出，我们需要引入 io 输入/输出库到当前作用域。 io 库来自于标准库，也被称为std：

fn main() {
    // fn 声明了一个新函数，（）表明没有参数，{作为函数体的开始。
    println!("GUess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);   // rand::thread_rng().gen_range(1..101); 生成1~101之间的随机数，不包括101。
    println!("The secret number is {}", secret_number);
    println!("Please enter your guess...");
    loop {
        let mut guess = String::new(); // 创建一个储存用户输入的变量, 将他绑定到一个新的String空实例上。
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {}", guess); // 里面的{}是预留的特定位置的占位符

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println! ("You Win!");
                break;
            }
        }
    }
}