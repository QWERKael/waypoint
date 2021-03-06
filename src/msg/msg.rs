use core::result::Result;
use std::io::{Error, Read, Write, BufWriter, BufReader};
use std::net::{TcpListener, TcpStream};

pub fn serv(address: String) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let stream = stream?;
        let mut reader = BufReader::new(stream);
        loop {
            let buf = &mut [0;3];
            let size = reader.read(buf)?;
            let s = String::from_utf8_lossy(buf);
            println!("获取到 {} bytes的字符串：{}", size, s);
            if size < 3 { break; }
        }
        println!("获取完成！");
    }
    Ok(())
}

pub fn send(address: String, buf: String) -> Result<(), Error> {
    let stream = TcpStream::connect(address)?;
    let mut writer = BufWriter::new(stream);
    let size = writer.write(buf.as_bytes())?;
    println!("发送 {} 的字符：{}", size, buf);
    Ok(())
}
