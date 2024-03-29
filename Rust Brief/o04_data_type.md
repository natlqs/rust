# 第 4 章 Rust 的数据类型
类型，我们先说一下现实中的菜系吧，
鲁菜、川菜、粤菜 、苏菜 、闽菜 、浙菜 、徽菜 、湘菜，都有不同的口味，当说到哪一个体系的菜的时候，你会知道它的特点，并且适合哪些人去吃。

那说回到计算机，数据类型也是一样的，就是存储和运算，并且要检查和保证这个数据在这个类型中是有效的。

Rust 是一个静态的严格数据类型的语言。每个值都有唯一的数据类型，要么是整型，要么是浮点型等等。

Rust 语言在赋值时并不强制要求指定变量的数据类型，Rust 编译器可以根据分配给它的值自动推断变量的数据类型。

## 声明变量
Rust 语言使用 let 关键字来声明和定义一个变量。

```rust
let 变量名=值
```

先粗略带过变量的声明，后面的章节我们会详细介绍。

```rust
fn main(){
    let food = "清蒸螃蟹";  // string 字符串类型
    let price = 366;       // float 类型
    let checked = true;   // boolean 类型

    println!("food is:{}", food); //输出 food is:清蒸螃蟹
    println!("price is:{}", price);//输出 price is:366
    println!("checked is :{}", checked);//输出 checked is :true
}
```

上面的代码中，我们并没有为每一个变量指定它们的数据类型。Rust 编译器会自动从 等号 = 右边的值中推断出该变量的类型。例如 Rust 会自动将 双引号 阔起来的数据推断为 字符串，把没有小数点的数字自动推断为 整型。把 true 或 false 值推断为 布尔类型。

## 基本数据类型

Rust 语言中有四种标量数据类型：

- 整型
- 浮点型
- 布尔类型
- 字符类型