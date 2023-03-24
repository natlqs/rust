# 31.智能指针
Rust 可以在 堆 上存储数据。Rust 语言中的某些类型，如 向量 `Vector` 和 字符串对象 `String` 默认就是把数据存储在 堆 上的。

Rust 语言把指针封装在如下两个特质`Trait`中。

特质名	|包	|Description|
---|---|---|
Deref|	std::ops::Deref|	用于创建一个只读智能指针，例如 *v
Drop|	std::ops::Drop|	智能指针超出它的作用域范围时会回调该特质的 drop() 方法。 类似于其它语言的 析构函数。

当一个结构体实现了以上的接口后，它们就不再是普通的结构体了。

Rust 提供了在 堆 上存储数据的能力并把这个能力封装到了 `Box` 中。

这种把 栈 上数据搬到 堆 上的能力，我们称之为 装箱。

## Box 指针
Box 指针可以把数据存储在堆（heap）上，而不是栈（stack）上。这就是装箱（box），栈（stack）还是包含指向 堆（heap） 上数据的指针。

```rust
fn main() {
    let a = 6;           // 默认保存在 栈 上
    let b = Box::new(a); // 使用 Box 后数据会存储在堆上
    println!("b = {}", b);// 输出 b = 6
}
```
## 访问 Box 存储的数据
如果想访问 Box 存储的数据，可以使用 星号 *访问，这个操作叫做 `解引用`。 `星号 *`也叫 解引用符。

```rust
let price1 = 158;           // 值类型数据
let price2 = Box::new(price1); // price2 是一个智能指针，指向堆上存储的数据 158
println!("{}",158==price1);
println!("{}",158==*price2); // 为了访问 price2 存储的具体数据，需要解引用

//输出
true
true
```
> 158==price1,是基础类型的比较只是比较值是否相等，所以返回 true。  
> 158==*price2,price2 是一个智能指针，是引用类型，想访问到具体的值，就要对 price2 进行解引用的操作。

## Deref 特质
实现 `Deref` 特质需要我们实现 `deref()` 方法。deref() 方法返回一个指向结构体内部数据的指针。

```rust
struct CustomBox<T>{
    value: T
}

impl<T> CustomBox<T> {
    fn new(v:T)-> CustomBox<T> {
        CustomBox{value:v}
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

let x = 666;
let y = CustomBox::new(x);  // 调用静态方法 new() 返回创建一个结构体实例

println!("666==x is {}",666==x);
println!("666==*y is {}",666==*y);  // 解引用 y
println!("x==*y is {}",x==*y);  // 解引用 y
//输出
666==x is true
666==*y is true
x==*y is true
```

## Drop 特质
`Drop Trait` 只有一个方法 `drop()`。当实现了 `Drop Trait` 的结构体，在超出了它的作用域范围时会触发调用 `drop()` 方法。

```rust
impl<T> Drop for CustomBox<T>{
    fn drop(&mut self){
        println!("drop CustomBox 对象!");
    }
}
```
再次运行以上的代码。

```rust
输出 drop CustomBox 对象!
```
我们在 堆（ heap ） 上创建了一个对象，该对象是 y。