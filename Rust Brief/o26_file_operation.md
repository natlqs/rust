# 26.文件操作

Rust语言使用结构体`File`来描述/展现一个文件。

所有对结构体`File`的操作方法都会返回一个`Result`枚举。

以下是一些常用的文件方法

模块	|方法	|说明|
---|---|---|
std::fs::File|	open()|	静态方法，以 只读 模式打开文件|
std::fs::File|	create()|	静态方法，以 可写 模式打开文件。 如果文件存在则清空旧内容 如果文件不存在则新建
std::fs::remove_file|	remove_file()|	从文件系统中删除某个文件|
std::fs::OpenOptions|	append()|	设置文件模式为 追加|
std::io::Writes|	write_all()|	将 buf 中的所有内容写入输出流|
std::io::Read|	read_to_string()|	读取所有内容转换为字符串后追加到 buf 中|
## 打开文件

模块提供了静态方法 `open()` 用于打开一个文件并返回文件句柄。

```rust
let file = std::fs::File::open("data.txt").unwrap();
println!("文件打开成功\n:{:?}",file);
```

文件打开成功:File { fd: 3, path: “/Users/monster/Github/rust_edu/file26/data.txt”, read: true, write: false }

## 创建文件
```rust
let file = std::fs::File::create("data2.txt").expect("创建失败");
println!("文件创建成功:{:?}",file);
```

文件创建成功:File { fd: 4, path: “/Users/monster/Github/rust_edu/file26/data2.txt”, read: false, write: true }

## 删除文件
使用 `remove_file()` 方法删除。

```rust
fs::remove_file("data.txt").expect("无法删除文件");
println!("文件已删除");
```
## 追加内容
```rust
let mut file = OpenOptions::new().append(true).open("data2.txt").expect("失败");
file.write("\nwww.go-edu.cn".as_bytes()).expect("写入失败");
println!("\n数据追加成功");
```

函数 append() 用于将文件的打开模式设置为 追加。

## 写入所有内容
```rust 
file.write_all("Rust".as_bytes()).expect("创建失败");
file.write_all("\nRust".as_bytes()).expect("创建失败");
println!("数据已写入完毕");
```
输出 数据已写入完毕

>注意： write_all() 方法并不会在写入结束后自动写入换行符 \n。

## 读取内容
```rust
let mut file = std::fs::File::open("data2.txt").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
print!("{}", contents);
//输出
www.go-edu.cnRust
Rust
```
