fn main() {
    let spend;
    {
        let x = 2;
        spend = x*x;
    }

    println!("spend:{}", spend);

    let spend2;
    // println!("spend2:{}", spend2);

    spend2 = 1;
    println!("another binding:{}", spend2);

}
