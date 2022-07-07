#[warn(unused_must_use)]
#[warn(unused_imports)]
use std::str;
use std::thread;
use std::thread::JoinHandle;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("服务端启动");
    let server_result = TcpListener::bind::<String>("127.0.0.1:6666".into());
    if server_result.is_err() {
        let error = server_result.err().unwrap();
        print!("{} {:#?}", "服务端开始监听时发生错误:", error);
        return;
    }
    let server = server_result.unwrap();
    println!("服务端开始监听");
    let mut handler_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in server.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("开始一次会话: {:#?}", stream);
                let handler = thread::spawn(move || {                    
                    loop {
                        let mut buffer = [0; 1024];
                        stream.read(&mut buffer).unwrap();
                        let messge = String::from_utf8_lossy(&buffer[..]);
                        println!("从客户端收到数据：{}", messge);
                        stream.write(format!("echo: {}", messge).as_bytes());

                        if messge.starts_with("10") {
                            println!("结束一次会话: {:#?}", stream);
                            break;
                        }
                    }                             
                });                
                handler_vec.push(handler);                
            }
            Err(error) => {
                print!("{} {:#?}", "获取客户端数据发生错误:", error);
            }
        }
    }
    for handler in handler_vec {
        handler.join().unwrap();
    }    
    println!("服务端监听任务结束");
}
