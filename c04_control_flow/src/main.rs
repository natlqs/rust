fn main() {

// if 表达式
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


// 使用 else if 处理多重条件
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


// 在 let 语句中使用if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is : {}", number);



// while 条件循环
// 在程序中计算循环的条件也很常见。当条件为真，执行循环。当条件不再为真，调用 break 停止循环。这个循环类型可以通过组合 loop、if、else 和 break 来实现；
// 如果你喜欢的话，现在就可以在程序中试试。然而，这个模式太常用了，Rust 为此内置了一个语言结构，它被称为 while 循环。示例 使用了 while 来程序循环 3 次，每次数字都减 1。接着在循环结束后，打印出另一个信息并退出。
    let mut number = 3;
    while number != 0{
        println!("{}", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");



//  使用for遍历集合 
// 可以使用 while 结构来遍历集合中的元素，比如数组。例如，示例的循环打印数组 a 中的每个元素。
// 我们增强了代码安全性，并消除了可能由于超出数组的结尾或遍历长度不够而缺少一些元素而导致的 bug。
// 使用 for 循环的话，就不需要惦记着在改变数组元素个数时修改其他的代码了。
// for 循环的安全性和简洁性使得它成为 Rust 中使用最多的循环结构。即使是在想要循环执行代码特定次数时，例如使用 while 循环的倒计时例子，大部分 Rustacean 也会使用 for 循环。
// 这么做的方式是使用 Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

// 下面是一个使用 for 循环来倒计时的例子，它还使用了一个我们还未讲到的方法，rev，用来反转区间（range）:
    for number in (1..4).rev(){
        println!("{}!", number);
    }
    println!("LIFTOFF!");

// 使用 loop重复执行代码, loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。
    loop {
        println!("again!"); // 当运行这个程序时，我们会看到连续的反复打印 again!，直到我们手动停止程序。
        //大部分终端都支持一个快捷键 ctrl-c 来终止一个陷入无限循环的程序Rust 也提供了一种从代码中跳出循环的方法。
        //可以使用 break 关键字来告诉程序何时停止循环。。
        break;
    }


// 使用循环标签，然后将标签与break或continue一起使用, 外层循环有一个标签 counting_up，它将从 0 数到 2。
// 没有标签的内部循环从 10 向下数到 9。第一个没有指定标签的 break 将只退出内层循环。break 'counting_up; 语句将退出外层循环。这个代码打印:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);


// 从循环返回
// 你可能会需要将操作的结果从循环中传递给其它的代码。为此，你可以在用于停止循环的 break 表达式添加你想要返回的值；该值将从循环中返回，以便您可以使用它.
// 在循环之前，我们声明了一个名为 counter 的变量并初始化为 0。接着声明了一个名为 result 来存放循环的返回值。在循环的每一次迭代中，我们将 counter 变量加 1，接着检查计数是否等于 10。
// 当相等时，使用 break 关键字返回值 counter * 2。循环之后，我们通过分号结束赋值给 result 的语句。最后打印出 result 的值，也就是 20。
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is  {}", result);
}
