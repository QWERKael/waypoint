use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::env;


fn serv(address: String) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let buf = &mut "".to_string();
        let size = stream.read_to_string(buf)?;
        println!("获取到 {} 的字符串：{}", size, buf);
    }
    Ok(())
}

fn send(address: String, buf: String) -> Result<(), Error> {
    let mut stream = TcpStream::connect(address)?;
    let size = stream.write(String::from(buf.clone()).as_bytes())?;
    println!("发送 {} 的字符：{}", size, buf);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = env::args();
    println!("{:?}", args);
    let mut args = args.skip(1);
    let flag = args.next().unwrap_or("".to_string());
    match flag.as_str() {
        "serv" => {
            let address = args.next().unwrap_or("0.0.0.0:8888".to_string());
            serv(address)?;
        }
        "send" => {
            let address = args.next().unwrap_or("127.0.0.1:8888".to_string());
            let buf = args.next().unwrap_or("没有输入字符串".to_string());
            send(address, buf)?;
        }
        _ => println!("未识别的指令：{}！", flag)
    }
    Ok(())
}
