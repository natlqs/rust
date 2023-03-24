use std::{thread, time::Duration};

fn main(){
    let handler = thread::spawn(||{
        for i in 1..10{
            println!("子线程{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //主线程
    for i in 1..5{
        println!("主线程{}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}