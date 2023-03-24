# 30.错误处理
Rust 语言也有错误这个概念，而且把错误分为两大类：可恢复 和 不可恢复，相当于其它语言的 异常 和 错误。

Name	|描述|
---|---|
Recoverable     |可以被捕捉，相当于其它语言的异常 Exception  |
UnRecoverable	|不可捕捉，会导致程序崩溃退出                   |

## panic!() 不可恢复错误
panic!() 程序立即退出，退出时调用者抛出退出原因。

一般情况下，当遇到不可恢复错误时，程序会自动调用 panic!()。
```rust
fn main() {
    panic!("出错啦");
    println!("Hello Rust"); // 不可能执行的语句
}
//输出
thread 'main' panicked at '出错啦', src/main.rs:2:5


let v = vec!["Go语言极简一本通","Go语言微服务架构核心22讲","从0到Go语言微服务架构师"];
v[5]; // 因为超出了数组的长度，所以会触发不可恢复错误
```

## Result 枚举和可恢复错误
枚举的定义如下：
```rust
enum Result<T,E> {
   OK(T),
   Err(E)
}
```

`OK(T)` T OK 时作为正常返回的值的数据类型。

`Err(E)` E Err 时作为错误返回的错误的类型。

```rust
let f = File::open("abc.jpg"); //文件不存在，因此值为 Result.Err
println!("{:?}",f);

//输出
Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
```
## unwrap() 和 expect()
unwrap() 函数的原型如下
```rust
unwrap(self):T
```
unwrap是 Result<T, E>的方法，在实例上调用此方法时，如果是 Ok 枚举值，就会返回 Ok 中的对象，如果是 Err 枚举值，在运行时会 panic，报错信息是 format!(“{}”, error)。其缺点是，如果在不同地方都使用 unwrap，运行时出现 panic 的时候。

```rust
fn is_even(no:i32)->Result<bool,String> {
    return if no % 2 == 0 {
        Ok(true)
    } else {
        Err("输入值，不是偶数".to_string())
    }
}

let result = is_even(6).unwrap();
println!("结果 {}",result);
//输出 结果 true

let result = is_even(11).unwrap();
println!("结果 {}",result);
//输出 thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "输入值，不是偶数"'
```
函数 expect() 的原型如下

```rust
expect(self,msg:&str):T
```
expect方法的作用和unwrap类似，区别在于，expect方法接受msg: &str作为参数，它在运行时的panic信息为format!("{}: {}", msg, error)，使用expect时，可以自定义报错信息，因此出现panic时比较容易定位。
```rust
let f = File::open("abc.txt").expect("无法打开该文件"); // 文件不存在
//输出 thread 'main' panicked at '无法打开该文件: Os { code: 2, kind: NotFound, message: "No such file or direct
```