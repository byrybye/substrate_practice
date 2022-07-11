
/**
@author phb
@date 2022-07-07
@description 简单网络功能实现 server 端
启动服务端 cargo run --bin server
启动客户端 cargo run --bin client
*/

use std::thread;
use std::{
    io::{Read, Write},
    net::{TcpListener},
};

fn main() {
    println!("服务端启动");
    //绑定ip 和 端口 开始监听
    let server_result = TcpListener::bind::<String>("127.0.0.1:6666".into());
    //如果发生错误
    if server_result.is_err() {
        //获取错误信息并打印
        let error = server_result.err().unwrap();
        print!("{} {:#?}", "服务端开始监听时发生错误:", error);
        //结束程序运行
        return;
    }
    //获取 服务端监听器
    let server = server_result.unwrap();
    println!("服务端开始监听");
    //建立 vec 容器存储 线程句柄
    let mut handler_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    //获取一个链接
    for stream in server.incoming() {
        //用match 匹配处理
        match stream {
            Ok(mut stream) => {
                println!("开始一次会话: {:#?}", stream);
                //开启线程 处理接收的信息
                let handler = thread::spawn(move || {                    
                    //使用循环处理一次连接 会话
                    loop {                     
                        //声明数组用来从stream里读取数据   
                        let mut buffer = [0; 1024];
                        //将数据从stream读到buffer中
                        stream.read(&mut buffer).unwrap();
                        //将buffer内容转成字符串
                        let messge = String::from_utf8_lossy(&buffer[..]);
                        println!("从客户端收到数据：{}", messge);
                        //向客户端发送回声消息
                        let _ = stream.write(format!("echo: {}", messge).as_bytes());
                        //如果消息开头是 10 那么 结束一次会话，跳出循环
                        if messge.starts_with("3") {
                            println!("结束一次会话: {:#?}", stream);
                            break;
                        }
                    }                             
                });                
                handler_vec.push(handler);                
            }
            //如果发生错误 打印错误信息
            Err(error) => {
                print!("{} {:#?}", "获取客户端数据发生错误:", error);
            }
        }
    }
    //在主线程等待 子线程结束
    for handler in handler_vec {
        handler.join().unwrap();
    }    
    //println!("服务端监听任务结束");
}
