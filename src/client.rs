use std::thread;
use std::time::Duration;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    println!("客户端启动");
    //客户端 连接服务端
    let client_result = TcpStream::connect::<String>("127.0.0.1:6666".into());
    //对连接结果做匹配模式处理
    match client_result {
        //如果连接成功
        Ok(mut client) => {
            println!("客户端连接成功");
            //循环10次 消息
            for i in 0..10 {
                println!("第{}次，开始发送", i + 1);
                //构建消息内容
                let message = format!("{}：hello world", i + 1);
                println!("消息内容：{}", message);
                //发送消息
                let _ = client.write(message.as_bytes());
                println!("第{}次，发送完成", i + 1);
                //暂停一秒 方便观察
                thread::sleep(Duration::from_secs(1));
                //声明数组用于读取数据
                let mut buffer = [0; 1024];
                //读取数据
                client.read(&mut buffer).unwrap();
                println!("从服务端收到数据: {}", String::from_utf8_lossy(&buffer[..]));
            }
            println!("客户端发送任务完成");
        }
        //如果发生错误，打印错误信息，退出程序
        Err(error) => {
            println!("{} {:#?}", "客户端连接时发生错误:", error);
            return;
        }
    }
}
