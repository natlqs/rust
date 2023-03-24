# 变量绑定
变量绑定默认是不可变的（immutable），但加上 mut 修饰语后变量就可以改变。

## 作用域和遮蔽
变量绑定有一个作用域（scope），它被限定只在一个代码块（block）中生存（live）。 代码块是一个被 {} 包围的语句集合。另外也允许变量遮蔽（variable shadowing）。

```rust
// 此绑定生存于 main 函数中
let spend = 1;
// 这是一个代码块，比 main 函数拥有更小的作用域
{
    // 此绑定只存在于本代码块
    let target = "面向加薪学习";

    println!("inner short: {}", target);

    // 此绑定*遮蔽*了外面的绑定
    let spend = 2.0;

    println!("inner long: {}", spend);
}
// 代码块结束

// 报错！`target` 在此作用域上不存在
// error[E0425]: cannot find value `target` in this scope
println!("outer short: {}", target);

println!("outer long: {}", spend);

// 此绑定同样*遮蔽*了前面的绑定
let spend = String::from("学习时间1小时");

println!("outer spend: {}", spend);
```
## 变量先声明
可以先声明（declare）变量绑定，后面才将它们初始化（initialize）。但是这种做法很 少用，因为这样可能导致使用未初始化的变量。
编译器禁止使用未经初始化的变量，因为这会产生未定义行为（undefined behavior）。

```rust
// 声明一个变量绑定
let spend;

{
    let x = 2;

    // 初始化一个绑定
    spend = x * x;
}

println!("spend: {}", spend);

let spend2;

// 报错！使用了未初始化的绑定
println!("spend2: {}", spend2);
// 改正 ^ 注释掉此行

spend2 = 1;

println!("another binding: {}", spend2);
```
## 冻结
资源存在使用的引用时，在当前作用域中这一资源是不可被修改的，称之为“冻结”。

```rust
let mut spend4 = Box::new(1);
let spend5 = &spend4;
spend4= Box::new(100);
println!("{}", spend4);
println!("{}", spend5);

报错如下
let spend5 = &spend4;
------- borrow of `spend4` occurs here
spend4= Box::new(100);
^^^^^^ assignment to borrowed `spend4` occurs here
println!("{}", spend4);
println!("{}", spend5);
------ borrow later used here
```