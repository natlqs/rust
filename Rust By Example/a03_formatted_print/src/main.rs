/// # 格式化输出
/*
* ## 打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：
    - fromat！：将格式化文本写到字符串
    - print! ：与format！类似，但是将文本输出到控制台（io::stdout).
    - println! ： 与print！类似，但输出结果追加一个换行符。
    - eprint! ： 与print！类似，但将文本输出到标准错误（io::stderr).
    - eprintln! ： 与eprint!类似，但输出结果追加一个换行符。

* ## std::fmt 包含多种 trait（特质）来控制文字显示，其中重要的两种trait的基本形式如下：
    - fmt::Debug: 使用{：？}标记，格式化文本以供调用使用
    - fmt::Display: 使用{}标记。以更优雅和友好的风格来格式化文本。
 */
fn main() {
    // 通常情况下， "{}"会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days",31);
    // 不追加后缀的话，31就自动称为i32类型。
    // 你可以添加后缀来改变31的类型（例如使用31i64 申明 31 为 i64类型）。

    // 用变量替换字符串由多种写法
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "alice", "bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over");

    // 可以在“：”后面指定特殊的格式。
    println!("{} of {:b} people known binary, the other half don't", 1, 2);

    // 可以按指定宽度来右对齐文本
    println!("{number:>width$}", number =1, width =6);

    // 可以在数字左边补0
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确。
    println!("My name is {0}, {1} {0}", "james", "bond");
    
    // 创建一个包含单个“i32” 的结构体（structure)。命名为“Structure".
    #[allow(dead_code)]
    struct Structure(i32);

    // 但是项结构体这样的自定义类型需要更富在的方式来处理。
   // println!("This struct '{:?}' won't print...", Structure(3));       这一行无法打印，因为struct需要特殊处理才能打印

}
