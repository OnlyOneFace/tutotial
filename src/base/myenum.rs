//枚举实践

#[derive(Debug)]
#[allow(dead_code)]
pub enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    MouseMove { x: i32, y: i32 },
    Write(String),
}
