use std::env;
use waypoint::{serv, send};

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
