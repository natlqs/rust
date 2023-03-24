# 32.包管理(Cargo)
Rust 内置了一个包管理器 cargo和 Rust 自带安装的，它也可以管理项目。

查看 cargo 版本
```rust
cargo --version
```
命令行输出
```rust
cargo 1.59.0
```
查看 Cargo 命令列表
```rust
cargo --list
```
命令行输出结果如下:

```rust
Installed Commands:
    b                    alias: build
    bench                Execute all benchmarks of a local package
    build                Compile a local package and all of its dependencies
    c                    alias: check
    check                Check a local package and all of its dependencies for errors
    clean                Remove artifacts that cargo has generated in the past
    clippy               Checks a package to catch common mistakes and improve your Rust code.
    config               Inspect configuration values
    d                    alias: doc
    doc                  Build a package's documentation
    fetch                Fetch dependencies of a package from the network
    fix                  Automatically fix lint warnings reported by rustc
    fmt                  Formats all bin and lib files of the current crate using rustfmt.
    generate-lockfile    Generate the lockfile for a package
    git-checkout         This subcommand has been removed
    help                 Displays help for a cargo subcommand
    init                 Create a new cargo package in an existing directory
    install              Install a Rust binary. Default location is $HOME/.cargo/bin
    locate-project       Print a JSON representation of a Cargo.toml file's location
    login                Save an api token from the registry locally. If token is not specified, it will be read from stdin.
    logout               Remove an API token from the registry locally
    metadata             Output the resolved dependencies of a package, the concrete used versions including overrides, in machine-readable format
    miri
    new                  Create a new cargo package at <path>
    owner                Manage the owners of a crate on the registry
    package              Assemble the local package into a distributable tarball
    pkgid                Print a fully qualified package specification
    publish              Upload a package to the registry
    r                    alias: run
    read-manifest        Print a JSON representation of a Cargo.toml manifest.
    report               Generate and display various kinds of reports
    run                  Run a binary or example of the local package
    rustc                Compile a package, and pass extra options to the compiler
    rustdoc              Build a package's documentation, using specified custom flags.
    search               Search packages in crates.io
    t                    alias: test
    test                 Execute all unit and integration tests and build examples of a local package
    tree                 Display a tree visualization of a dependency graph
    uninstall            Remove a Rust binary
    update               Update dependencies as recorded in the local lock file
    vendor               Vendor all dependencies for a project locally
    verify-project       Check correctness of crate manifest
    version              Show version information
    yank                 Remove a pushed crate from the index
```
以上是 Cargo 提供的命令，常用的如下:

命令	|说明|
---|---|
cargo new|	在当前目录下新建一个 cargo 项目
cargo check|	分析当前项目并报告项目中的错误，但不会编译任何项目文件
cargo build	|编译当前项目
cargo run|	编译并运行文件 src/main.rs
cargo clean|	移除当前项目下的 target 目录及目录中的所有子目录和文件
cargo update|	更新当前项目中的 Cargo.lock 文件列出的所有依赖