# 第 2 章 Rust 环境搭建
在 Mac 环境下，利用 brew 包管理，十分方便。

1. brew upgrade

2. brew install rustup-init

3. 进入/opt/homebrew/Cellar/rustup-init/1.24.3/bin

4. 运行 rustup-init

```shell
1) Proceed with installation (default) //默认安装
2) Customize installation //自定义安装
3) Cancel installation	//取消安装

我选的是第 1 个。
```

5. 屏幕出现成功字样 Rust is installed now. Great!

6. 最后执行这句，让环境变量生效 source $HOME/.cargo/env

7. 再打开终端

```
rustc -V
屏幕输出  rustc 1.59.0
出现上面的提示，证明你的 Rust 环境安装好了。
```