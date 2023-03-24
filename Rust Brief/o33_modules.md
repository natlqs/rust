# 33.模块(Modules)
我们常说 功能模块，就是用于将函数或结构体按照功能分组。也常常把相似的函数或者实现相同功能的或者共同实现一个功能的函数和结构体划分到一个模块中。

Rust 中的模块，类似 C++ 中的命名空间，Java 语言中的包。

在代码组织上，比模块更高级的是 crate ，一个crate 可以存放多个模块，在 Rust 语言中crate 是基本编译单元，分为 可执行二进制文件（包含 main 函数作为程序入口） 或者 一个库。

`crates.io` 是 Rust 官方提供的第三方包的地址。可以使用 `cargo install` 命令从 crates.io 上下载你所需要的 crate。

定义模块
```rust
mod module_name {
   fn function_name() {
      // 具体的函数逻辑
   }
}
```
1. module_name 要是一个合法的名称。

2. Rust 语言中的模块默认是私有的。

3. 如果一个模块或者模块内的函数需要导出为外部使用，则需要添加 pub 关键字。

4. 私有的模块不能为外部其它模块或程序所调用。

5. 私有模块的所有函数都必须是私有的，而公开的模块，则即可以有公开的函数也可以有私有的函数。

```rust
//公开的模块
pub mod public_module {
   pub fn public_function() {
      // 公开的方法
   }
   fn private_function() {
      // 私有的方法
   }
}
//私有的模块
mod private_module {

   // 私有的方法
   fn private_function() {
   }
}
```
## use 关键字
```rust
use 公开的模块名::函数名;
```
在根目录下，执行 cargo new –lib mylib,创建类库。

```rust
pub mod add_salary {
    pub fn study(name:String) {
        println!("面向加薪学习 {}",name);
    }
}

第一步 进入 mylib目录执行cargo build
第二步 打开根目录 Cargo.toml
[dependencies]
mylib={ path="../module_demo/mylib" }
第三步 在main.rs中修改
use mylib::add_salary::study;

fn main(){
    study("从0到Go语言微服务架构师".to_string());
}

use AddSalary::study;

fn main() {
    study("从0到Go语言微服务架构师".to_string());
}
//输出
面向加薪学习 从0到Go语言微服务架构师
```
Rust 允许一个模块中嵌套另一个模块，换种说法，就是允许多层级模块。

```rust
pub mod mod1 {
   pub mod mod2 {
      pub mod mod3 {
         pub fn 方法名(参数) {
            //代码逻辑
         }
      }
   }
}
```
调用或使用嵌套模块的方法使用两个冒号 (::) 从左到右拼接从外到内的模块即可

```rust
use mod1::mod2::mod3::方法名;

fn main() {
    方法名();
}
```