fn main() {
    let result = is_even(6).expect("aa");
    println!("结果{}", result);

    let result = is_even(11).expect("bb");
    println!("结果{}", result);

}

fn is_even(no:i32) ->Result<bool, String> {
    return if no % 2 == 0{
        Ok(true)
    } else {
        Err("输入值，不是偶数".to_string())
    }
}