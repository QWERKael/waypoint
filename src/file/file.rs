use core::result::Result;
use std::io::{Error, Read, Write, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::str;
use std::convert::TryInto;

pub fn serv(address: String) -> Result<(), Error> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let stream = stream?;
        let mut reader = BufReader::new(stream);
        let buf = &mut [0;4096];
        let read_size = reader.read(buf)?;
        println!("获取到buf");
        let filename_bytes_len = u64::from_be_bytes(buf[..8].try_into().unwrap()) as usize;
        println!("获取到文件名长度：{}", filename_bytes_len);
        let filename = &buf[8..filename_bytes_len+8];
        let filename = str::from_utf8(&filename).unwrap_or("UnknownName");
        println!("获取到文件名：{}", filename);
        let mut file = File::create(filename)?;
        let buf_size = file.write(&buf[filename_bytes_len + 8..read_size])?;
        println!("写入 {} bytes", buf_size);
        break;
    }
    println!("传输结束！");
    Ok(())
}

pub fn send(address: String, path: String) -> Result<(), Error> {
    println!("开始发送");
    let stream = TcpStream::connect(address)?;
    // println!("准备writer");
    let mut writer = BufWriter::new(stream);
    // println!("准备打开文件：{}", path);
    let mut file = File::open(path.clone())?;
    // println!("打开文件：{}", path);
    let filename = path.split("/").collect::<Vec<&str>>().pop().unwrap_or("ErrName");
    // println!("获取到文件名：{}", filename);
    let filename_bytes = filename.as_bytes();
    let filename_bytes_len = filename_bytes.clone().len() as u64;
    // println!("准备发送文件名");
    let _ = writer.write(&filename_bytes_len.to_be_bytes());
    let _ = writer.write(filename_bytes);
    println!("发送文件名：{}", filename);
    let mut buf = [0u8; 1024];
    loop {
        match file.read(&mut buf) {
            Ok(read_size) => {
                println!("读取 {} bytes", read_size);
                if read_size == 0 { break; }
                let send_size = writer.write(&buf[0..read_size])?;
                println!("发送 {} bytes", send_size);
            }
            Err(e) => {
                println!("发生了一个错误：{}", e);
                break;
            }
        }
    }
    println!("发送结束!");
    Ok(())
}
