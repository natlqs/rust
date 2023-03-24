# 类型系统
## 类型转换
Rust 不提供原生类型之间的隐式类型转换，但可以使用 as 关键字进行显式类型转换。  
整型之间的转换大体遵循 C 语言的惯例，除了 C 会产生未定义行为的情形。在 Rust 中所 有整型转换都是定义良好的。

```rust
let spend = 1;
// 错误！不提供隐式转换
// error[E0308]: mismatched types
// let cost: f64 = spend;

// 可以显式转换
let cost = spend as f64;
println!("转换: {} -> {}", spend, cost);
```
## 字面量
对数值字面量，只要把类型作为后缀加上去，就完成了类型说明。比如指定字面量 42 的 类型是 i32，只需要写 42i32。

无后缀的数值字面量，其类型取决于怎样使用它们。如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64。

```rust
// 带后缀的字面量，其类型在初始化时已经知道了。
 let x = 1u8;
 let y = 2u32;
 let z = 3f32;

 // 无后缀的字面量，其类型取决于如何使用它们。
 let i = 1;
 let f = 1.0;
 ```
## 类型推断
Rust 的类型推断引擎是很聪明的，它不只是在初始化时看看右值（r-value）的 类型而已，它还会考察变量之后会怎样使用，借此推断类型。

```rust
// 因为有类型说明，编译器知道类型是 u8。
let study = String::from("从0到Go语言微服务架构师");

// 创建一个空向量（vector，即不定长的，可以增长的数组）。
let mut vec = Vec::new();
// 现在编译器还不知道 `vec` 的具体类型，只知道它是某种东西构成的向量（`Vec<?>`）

// 在向量中插入元素。
vec.push(study);
// 现在编译器知道 `vec` 是 String 的向量了（`Vec<String>`）。
println!("{:?}", vec);
```
## 别名
可以用 type 语句给已有的类型取个新的名字。类型的名字必须遵循驼峰命名法（像是 CamelCase 这样），否则编译器将给出警告。原生类型是例外，比如： usize、f32，等等。别名的主要用途是避免写出冗长的模板化代码。
type 新名字 = 原名字;

```rust
type MyU64 = u64;
type OtherU64 = u64;
type ThirdU64 = u64;
fn main(){
    let MyU64: MyU64 = 5 as ThirdU64;
    let otherU64: OtherU64 = 2 as ThirdU64;
    println!(
        "{} MyU64 + {} OtherU64es = {} unit?",
        MyU64,
        otherU64,
        MyU64 + otherU64
    );
}
```
注意类型别名并不能提供额外的类型安全，因为别名并不是新的类型。