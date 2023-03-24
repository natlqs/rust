use std::{fs::{self, OpenOptions}, io::{Write, Read}};

fn main(){
    // 打开文件
    let file = std::fs::File::open("data.txt").unwrap();
    println!("文件打开成功: {:?}", file);
    
    // 创建文件
    let file = std::fs::File::create("data2.txt").expect("创建失败");
    println!("文件创建成功：{:?}", file);

    // 删除文件
    fs::remove_file("data2.txt").expect("无法删除文件");
    println!("文件已删除");

    // 追加内容
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("失败");
    file.write("rust".as_bytes()).expect("写入失败");
    println!("\n数据追加成功");

    // 写入所有内容
    file.write_all("rust".as_bytes()).expect("创建失败");
    file.write_all("\nrust".as_bytes()).expect("创建失败");
    println!("数据已写入完毕");

    // 读取内容
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!{"{}", contents};
}

