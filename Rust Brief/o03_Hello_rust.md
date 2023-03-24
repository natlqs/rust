# 第三章 hello_rust
第一个 Rust 程序。

1. 先创建一个目录 hello_rust。

2. 可以使用趁手的编辑（jetbrains 系列可以安装 rust 插件，或者是 vscode 的 rust 插件都可以）。

3. 创建 rust 源代码文件，它是用 .rs 作为扩展名的。

4. 在 hello.rs 中，输入

```rust
fn main(){
    println!("Hello Rust!")
}
```

>fn - Rust 语言使用 fn 关键字定义函数。

>main() 函数是一个预定义的主函数，充当 Rust 程序的入口点，每个语言都会有自己的 main() 函数。

>println!() 是 Rust 语言中的一个 预定义的宏。它用于将传递给它的参数输出到 标准输出。

​ 注：Rust 语言中的 宏 都会以 感叹号 ( ! ) 结尾。以后看到以 ! 结尾的类似函数调用，都是 宏调用。Rust 提供了一个功能非常强大的 宏 体系，通过这些 宏，我们可以很方便的进行 元编程。Rust 中的 宏 有点类似于 函数。可以将 宏 理解为 函数的加强版。

5. 使用 rustc hello.rs,编译出一个以 hello 为名字的二进制的可执行文件
6. 运行 ./hello,屏幕输出 Hello Rust!