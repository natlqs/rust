# 第 5 章 整型
最大值 std::u128::MAX，它的值是 340282366920938463463374607431768211455

最小值 std::i128::MIN，它的值是 -170141183460469231731687303715884105728

整数可以分为 **有符号整型** 和 **无符号整型**

- 有符号整型，英文 `signed`，既可以存储正数，也可以存储负数。

- 无符号整型，因为 `unsigned`，只能存储正数。

大小	|有符号	|无符号|
---|---|---|
8 bit	|i8	|u8
16 bit	|i16	|u16
32 bit	|i32	|u32
64 bit	|i64	|u64
128 bit |i128	|u128
Arch	|isize	|usize

按照存储空间来说，整型可以进一步划分为 1字节、2字节、4字节、8字节、16字节。

1 字节 = 8 位，每一位能只能存储二进制 0 或 1，因此每一个字节能够存储的最大数字是 256，而最小数字则是 -127。

整型的长度还可以是 `arch`。arch 是由 CPU 构架决定的大小的整型类型。大小为 arch 的整数在 x86 机器上为 32 位，在 x64 机器上为 64 位。

i32 是默认的整型。

```rust
let price = 100;    // i32 默认
let price2:u32 = 200;
let price3:i32 = -300;
let price4:isize = 400;
let price5:usize = 500;

println!("price is {}", price);
//输出 price is 100

println!("price2 is {} and price3 is {}", price3, price2);
//输出 price2 is -300 and price3 is 200

println!("price4 is {} and price5 is {}", price4, price5);
//输出 price4 is 400 and price5 is 500
```

如果类型和值不匹配，编译不会通过，并且报错。

```rust
let price6:i32=66.66
编译器会提示：mismatched types [E0308] expected `i32`, found `f64`
```

## 整型取值范围
有符号整型 能够存储的最小值为 -(2^(n-1)，能够存储的最大值为 2^(n-1) -1。

无符号整型 能够存储的最小值为 0，能够存储的最大值为 2^n - 1。

其中 n 是指数据类型的大小。（上面表格里的第一列）

整型 i8 ，能够存储的最小值为 -(2^(8-1)) = -128。最大值为 (2^(8-1)-1) = 127。

## 整型溢出
我们已经计算了 i8 的最大值是 127。我给一个更大的数值会如何呢？

```rust
let price7:i8=192;
println!("price7 is {}", price7);

报错如下：
16 |     let price7:i8=192;
 |                   ^^^
 |
 = note: `#[deny(overflowing_literals)]` on by default
 = note: the literal `192` does not fit into the type `i8` whose range is `-128..=127`
 = help: consider using the type `u8` instead
 ```

很明确的告诉你超出了 i8 的范围