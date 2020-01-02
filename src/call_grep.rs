use std::env::args;
use std::process::{Command, Stdio};

pub fn call_grep() {
    let mut arg_iter = args();
//    如果没有参数的话直接panic
    arg_iter.next();
    let pattern = arg_iter.next().unwrap_or("main".to_string());
    let pt = arg_iter.next().unwrap_or("./".to_string());
    let child = Command::new("/usr/bin/grep")
        .arg("-n")
        .arg("-r")
        .arg(&pattern)
        .arg(&pt)
        .stdout(Stdio::piped())
        .spawn().unwrap();
    std::thread::sleep_ms(1000);
    println!("{}", "计算很费时间......");
    let out = child.wait_with_output().unwrap();
    let out_str = String::from_utf8_lossy(&out.stdout);
    for line in out_str.split("\n"){
        println!("{}", line);
    }
}