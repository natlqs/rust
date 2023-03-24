# match 匹配
Rust 通过 match 关键字来提供模式匹配，和 C 语言的 switch 用法类似。第一个匹配分支会被比对，并且所有可能的值都必须被覆盖。

## 解构枚举
见 22 小节内容

## 解构指针和引用
对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念是不同的，和 C 那样的语言用法不一样。

    - 解引用使用 *
    - 解构使用 &、ref、和 ref mut
```rust
// 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let num = &100;

    match num {
        // 用 `&val` 这个模式去匹配 `num`
        &val => println!("&val 是: {:?}", val),
    }

    // 如果不想用 `&`，需要在匹配前解引用。
    match *num {
        val => println!("val 是: {:?}", val),
    }

    // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用。
    let ref num3 = 66;

    // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
    let num4 = 5;
    let mut mut_num4 = 7;

    // 使用 `ref` 关键字来创建引用。
    // 下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
    // 似乎看不出什么区别。但读者可以把 `println!` 中的 `r` 改成 `*r`，仍然能
    // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
    // 引用。
    match num4 {
        ref r => println!("num4 r is: {:?}", r),
    }

    // 类似地使用 `ref mut`。
    match mut_num4 {
        ref mut m => {
            // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
            *m += 10;
            println!("`mut_value`: {:?}", m);
        }
    }
```
## 解构结构体
```rust
struct Study {
    name: String,
    target: String,
    spend: u32,
}
fn main(){

let s = Study {
        name: String::from("从0到Go语言微服务架构师"),
        target: String::from("全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。"),
        spend: 3,
};

let Study {
        name: name,
        target: target,
        spend: spend,
} = s;

println!("name = {}, target = {},  spend = {} ", name, target, spend);
// name = 从0到Go语言微服务架构师, target = 全面掌握Go语言微服务落地，代码级一次性解决微服务和分布式系统。,  spend = 3
let s2 = Study {
        name: String::from("《Go语言极简一本通》"),
        target: String::from("学习Go语言，并且完成一个单体服务。"),
        spend: 5,
};
// 也可以忽略某些变量
let Study { name, .. } = s2;
println!("name = {}", name);
//name = 《Go语言极简一本通》
}
```