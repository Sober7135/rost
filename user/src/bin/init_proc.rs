#![no_std]
#![no_main]

use user_lib::{console::getchar, exec, fork, getpid, wait};

#[macro_use]
extern crate user_lib;

#[no_mangle]
unsafe fn main() -> i32 {
    println!("this is init_proc");
    let pid = fork();
    if pid == 0 {
        println!("I'm child, parent pid={}", pid);
        println!("test pid={}", getpid());
        exec("00power_3\0");
        println!("after exec 00power_3")
    } else {
        println!("I'm parent, pid={}", pid);
        println!("test pid={}", getpid());
        let mut code: i32 = 0;
        let ret = wait(&mut code);
        println!("ret={}, exit_code={}", ret, code);
        loop {
            let ch = getchar();
            println!("{}", ch as char);
        }
    }
    0
}