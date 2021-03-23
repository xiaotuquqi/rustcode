//服务器
use std::io::{self, prelude::*, BufReader, Write};
use std::net::TcpStream;
use std::str;
//导入标准库    
fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;//与客户端建立连接
    for _ in 0..10 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)//读取输入
            .expect("Failed to read from stdin");//错误处理
        stream
            .write(input.as_bytes())//向客户端写入
            .expect("Failed to write to stream");//错误处理
        
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();//定义vector
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");//错误处理
        println!("{}", 
            str::from_utf8(&buffer).expect("Could not write buffer as string"));
        println!("");
    }
    Ok(())
}

