# 类型转换
Rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 Into 两个 trait。

## From 和 Into
From 和 Into 两个 trait 是内部相关联的，实际上这是它们实现的一部分。如果我们能够从类型 B 得到类型 A，那么很容易相信我们也能把类型 B 转换为类型 A。

## From
From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能。

```rust
    let s1 = "从0到Go语言微服务架构师";
    let s2 = String::from(s1);


#[derive(Debug)]
struct MyNumber {
    num: i32,
}

impl From<i32> for MyNumber {
    fn from(item: i32) -> Self {
        MyNumber { num: item }
    }
}
fn main() {
    let my_number = MyNumber { num: 1 };
    println!("{:?}", my_number);
}
```

## Into
Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。不过考虑到我们免费获得了 Into，这点代价不值一提。

```rust
let spend = 3;
let my_spend: MyNumber = spend.into();
println!("{:?}", my_spend);
```
## 解析字符串
经常需要把字符串转成数字。完成这项工作的标准手段是用 parse 函数。

只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行。

```rust
let cost: i32 = "5".parse().unwrap();
println!("{}", cost);
```