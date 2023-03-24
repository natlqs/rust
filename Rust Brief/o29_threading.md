# 29.多线程
1. 现代的操作系统，是一个多任务操作系统，系统可以管理多个程序的运行，一个程序往往有一个或多个进程，而一个进程则有一个或多个线程。

2. 让一个进程可以运行多个线程的机制叫做多线程。

3. 一个进程一定有一个主线程，主线程之外创建出来的线程叫 子线程

多线程（并发）编程的一个重要思想就是 程序不同的部分可以同时独立运行互不干扰。

## 创建线程
```rust
std::thread::spawn()

//spawn() 函数的原型
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
```
参数 f 是一个闭包（closure ） 是线程要执行的代码。

```rust
fn main() {
    //子线程
    thread::spawn(|| {
        for i in 1..10 {
            println!("子线程 {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程
    for i in 1..5 {
        println!("主线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

>当主线程执行结束，子线程就自动结束。

thread::sleep() 会让线程睡眠一段时间，某个线程睡眠的时候会让出 CPU，可以让不同的线程交替执行，要看操作系统如何调度线程。

## join
上面的例子主线程结束后，子线程还没有运行完，但是子线程也结束了。如果想让子线程结束后，主线程再结束，我们就要使用Join 方法，把子线程加入主线程等待队列。

```rust
spawn<F, T>(f: F) -> JoinHandle<T>
//子线程
let handler = thread::spawn(|| {
    for i in 1..10 {
        println!("子线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});
// 主线程
for i in 1..5 {
    println!("主线程 {}", i);
    thread::sleep(Duration::from_millis(1));
}
handler.join().unwrap();
```
输出结果看 主线程让子线程执行完毕后，主线程才退出。

