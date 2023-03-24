fn main() {
    let s1 = "go";
    let s2 = String::from(s1);
    println!("{}", s2);
    let my_number = MyNumer{num:1};
    println!("{:?}", my_number);

    let spend = 3;
    let my_spend: MyNumer = spend.into();
    println!("{:?}", my_spend);

}

#[derive(Debug)]
struct  MyNumer{
    num:i32,
}

impl From<i32> for MyNumer {
    fn from(item: i32) -> Self {
        MyNumer { num: item }
    }
    
}