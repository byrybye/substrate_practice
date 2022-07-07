
use std::str;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[warn(unused_must_use)]
#[warn(unused_imports)]
fn main() {
    println!("客户端启动");
    let client_result = TcpStream::connect::<String>("127.0.0.1:6666".into());
    match client_result {
        Ok(mut client) => {
            println!("客户端连接成功");
            for i in 0..10 {
                println!("第{}次，开始发送", i + 1);
                let message = format!("{}：hello world", i + 1);
                println!("消息内容：{}", message);
                client.write(message.as_bytes());
                println!("第{}次，发送完成", i + 1);
                thread::sleep(Duration::from_secs(1));
                let mut buffer = [0; 1024];
                client.read(&mut buffer).unwrap();
                println!("从服务端收到数据: {}", String::from_utf8_lossy(&buffer[..]));
                
            }
            println!("客户端发送任务完成");
        }
        Err(error) => {
            println!("{} {:#?}", "客户端连接时发生错误:", error);
            return;
        }
    }
}
