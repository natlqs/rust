use std::ops::Deref;

fn main() {
    let x = 666;
    let y = CustomBox::new(x);      // 调用静态方法new()，返回创建一个结构体实例
    println!("666==x is {}",  666 == x);
    println!("666==*y is {}", 666 == *y);
    println!("x==*y is {}", x == *y);

}

struct CustomBox<T>{
    value: T
}

impl <T> CustomBox<T>{
    fn new(v:T) -> CustomBox<T>{
        CustomBox { value: v }
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;
    fn deref(&self) -> &T{
        &self.value
    }
}

impl <T>Drop for CustomBox<T> {
    fn drop(&mut self) {
        println!("drop CustomBox 对象");
    }
    
}