fn main() {
    let double = |x| {x *2};
    let add = |a, b| {a + b};
    let x = add(2, 4);
    println!("{}", x);

    let y = double(5);
    println!("{}", y);
    
    let v = 3;
    let add2 = |x| {v + x};
    println!("{}", add2(4));
}
