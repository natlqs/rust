use std::io;
fn main() {

    // 复合类型（Compound Type),可以将多个值组成一个类型。


    //元组类型
    //元组类型是将多个值组合到也给复合类型的一种基本方式。元组的长度是固定的：声明后，他们就无法增长或缩小。
    let tup: (i32, f64, u8) = (500, 6.4,1);
    println!("The value of the tup is: {:?}", tup);
    //想从元组中获取个别值，可以使用匹配来解构元组的一个值
    let(o, p, q) = tup;
    // 还可以使用一个句点（.）脸上要访问的值的索引来直接访问元组元素
    println!("{}, {}, {}",o, p, q);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The values are: {:?}, {}, {}", five_hundred, six_point_four, one);

    //数组类型
    // 数组的每个元素都必须是相同的类型。Rust中的数组具有固定长度
    let a = [1, 2, 3,4, 5, 6];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", 
    "September", "October", "November", "December"];
    //使用方括号编写数组的类型，其中包含每个元素的类型、分号，然后是数组中的元素数。
    let b:[i32;5] = [1, 2, 3, 4, 5];
    // 还可以通过指定初始值、后跟分号和方括号中的数组长度来初始化数组
    let c = [3; 5];
    //变量名为c的数组将包含5个元素，这些元素的值初始化为3， 这种写法与 let c = [3, 3, 3, 3, 3]; 效果相同，但是更简洁
    for i in a.iter() {
        println!("{}", i);}
    for j in months.iter() {
        println!("{}", j);}
    for k in b.iter() {
        println!("{}", k);}
    for l in c.iter() {
        println!("{}", l);}


    //无效的数组访问例子，如果尝试访问超出数组末位的数组元素，会导致运行时错误，程序退出并显示错误信息。
    let d = [1, 2, 3, 4, 5];
    println!("Please enter an array index:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect(" Failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = d[index];
    println!("The value of the element at index {} is : {}", index, element);
}