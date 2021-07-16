use core::result::Result;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn serv(address: String) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        let buf = &mut "".to_string();
        let size = stream.read_to_string(buf)?;
        println!("获取到 {} 的字符串：{}", size, buf);
    }
    Ok(())
}

pub fn send(address: String, buf: String) -> Result<(), Error> {
    let mut stream = TcpStream::connect(address)?;
    let size = stream.write(String::from(buf.clone()).as_bytes())?;
    println!("发送 {} 的字符：{}", size, buf);
    Ok(())
}
