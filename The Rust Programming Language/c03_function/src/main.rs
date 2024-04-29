fn another_function(x: i32, unit_label:char) {   //函数签名中，必须声明每个参数的类型。
    println!("The value of x, unit_label is: {} {}", x, unit_label);
}

fn five() ->i32 {      // 函数返回值的类型也被指定好，即 ->32。
    5
}

fn main() {
    println!("Hello, world!");      // 语句是执行一些操作但不返回值的指令。
    let mut _y = 6;  // 一个语句。
    // 语句不返回值，因此不能把let语句赋值给另一个变量，就像下面的代码尝试做的那样，会产生错误：
    //let x = (let y = 6);      会产生错误
    //正确的方法：
    let _x = _y = 6;      //这样x 和y的值都是6。

    //表达式：会计算出一个值，如5+6。 函数调用是一个表达式，宏调用是一个表达式。
    let z = {
        let x = 3;
        x + 1
    };
    println!("The value of x is {}", z);
    
    //调用函数
    another_function(5, 'h'); // 可以使用函数名后跟圆括号来调用我们定义过的任意函数。源码中another_function定义在main函数之前，也可以定义在之后，Rust不关心函数定义于何处，只要定义了就行。

    // 调用有返回值的函数
    let o = five();      
    println!("The value of o is {}", o);

}

