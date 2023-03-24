# 27.迭代器
**迭代器** 就是把集合中的所有元素按照顺序一个接一个的传递给处理逻辑。

`Iterator` 特质有两个函数：

一个是 `iter()`，用于返回一个 迭代器 对象，也称之为 项 ( items ) 。

一个是 `next()`，用于返回迭代器中的下一个元素。如果已经迭代到集合的末尾（最后一个项后面）则返回 `None`。
```rust
fn main() {
    let v = vec!["Go语言极简一本通", "Go语言微服务架构核心22讲", "从0到Go语言微服务架构师"];
    let mut it = v.iter();
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
    println!("{:?}",it.next());
}
//输出
Some("Go语言极简一本通")
Some("Go语言微服务架构核心22讲")
Some("从0到Go语言微服务架构师")
None
```

用 `for ... in` 语句遍历。

```rust
let iter = v.iter();
for item in iter{
   print!("{}\n",item);
}
```

方法	            |           描述    |
-----               |----                         |
`iter()`	        |   返回一个只读可重入迭代器，迭代器元素的类型为 `&T`
`into_iter()`       |	返回一个只读不可重入迭代器，迭代器元素的类型为 `T`
`iter_mut()`	    |   返回一个可修改可重入迭代器，迭代器元素的类型为 `&mut T`
