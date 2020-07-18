use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;

///read_io 输入读取
pub fn read_io() -> String {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            println!("{} bytes read", n);
            buffer
        }
        Err(error) => {
            println!("error: {}", error);
            String::from("")
        }
    }
}

pub fn std_out() {
    print!("请输入一个字符串：");
    io::stdout().flush().expect("无效输出");
}

pub fn file_open() {
    let path = Path::new(r"C:\Users\lcf\Desktop\hello.txt");
    let display = path.display();

    // 打开文件只读模式, 返回一个 `io::Result<File>` 类型
    let mut file = match File::open(&path) {
        // 处理打开文件可能潜在的错误
        Err(why) => {
            println!("couldn't open {}: {}", display, why);
            return;
        }
        Ok(file) => file,
    };

    // 文件输入数据到字符串，并返回 `io::Result<usize>` 类型
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => {
            println!("couldn't read {}: {}", display, why);
            return;
        }
        Ok(value) => print!("path={},read len ={},contains:\n{}\n", display, value, s),
    }
}

pub fn file_write() {
    let path = Path::new(r"C:\Users\lcf\Desktop\hello.txt");
    let display = path.display();

    // 打开文件只读模式, 返回一个 `io::Result<File>` 类型
    let mut file = match OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open(&path)
    {
        // 处理打开文件可能潜在的错误
        Err(why) => {
            println!("couldn't open {}: {}", display, why);
            return;
        }
        Ok(file) => file,
    };

    let text: &'static str =
        "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
这是测试文档";
    match file.write_all(text.as_bytes()) {
        Err(why) => {
            println!("couldn't read {}: {}", display, why);
            return;
        }
        Ok(_) => print!("path={}\n", display),
    }
}
