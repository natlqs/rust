fn main() {
    // 常量
    const THREE_HOURS_IN_SECONDS: usize = 60 * 60 * 30;  // 定义常量
    println!("{}", THREE_HOURS_IN_SECONDS);


    // 变量的可变性
    let mut x = 5;
    println!("The value of the x is: {}", x);
    x = 6;
    println!("The value of the x is: {}", x);

    // 遮蔽: 可以声明和前面变量具有相同名称的新变量。这是第一个变量被第二个变量遮蔽，一位置当我们使用变量时我们看到的会是第二个变量的值。
    // 我们可以使用相同的变量名并重复使用let关键字来遮蔽变量
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is :{}", y);
    }
    println!("The value of y is : {}", y);
    //遮蔽和将变量标记为 mut 的方式不同，因为除非我们再次使用 let 关键字，否则若是我们不小心尝试重新赋值给这个变量，我们将得到一个编译错误。
    //通过使用 let，我们可以对一个值进行一些转换，但在这些转换完成后，变量将是不可变的。

    //mut 和遮蔽之间的另一个区别是，因为我们在再次使用 let 关键字时有效地创建了一个新的变量，所以我们可以改变值的类型，但重复使用相同的名称。
    //例如，假设我们程序要求用户输入空格字符来显示他们想要的空格数目，但我们实际上想要将该输入存储为一个数字：
}

