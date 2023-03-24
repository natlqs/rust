// 迭代器就是把集合中的所有元素按照顺序一个接一个的传递给处理逻辑
// Iterator由两个特质函数：
// iter(), 用于返回一个迭代器对象，也称之为项（items）。
// next(), 用于返回迭代器的下一个元素，如果已经迭代到集合的末尾（最后一个项后面）则返回None。

fn main() {

    let v = vec!["Go语言极简一本通", "Go语言微服务架构核心22讲", "从0到Go语言微服务架构师"];
    let mut it = v.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());    
}
