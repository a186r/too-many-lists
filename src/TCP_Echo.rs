use std::net::{ToSocketAddrs, TcpListener, TcpStream};
use std::{io, thread};
use std::io::{Read, Write};
use std::intrinsics::panic_if_uninhabited;

pub fn server<A: ToSocketAddrs>(addr: A) -> io::Result<()>{
//    建立一个监听程序
    let listener = try!(TcpListener::bind(&addr));
//    这个程序一次只需要处理一个链接就好
    for stream in listener.incoming(){
//        通过match再次解包stream到
        match stream {
            Ok(mut st) => {
                let mut buf: Vec<u8> = vec![0u8; 1024];
                let rcount = try!(st.read(&mut buf));
                println!("{:?}", &buf[0..rcount]);
                let wcount = try!(st.write(&buf[0..rcount]));
                if rcount != wcount{
                    panic!("Not Fully Echo!, r={}, w={}", rcount, wcount);
                }
                buf.clear();
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

//    关闭掉server端的链接
    drop(listener);
    Ok(())
}

//准备一个模拟TCP短链接的客户端
pub fn client<A: ToSocketAddrs>(addr: A) -> io::Result<()>{
    let mut buf = vec![0u8; 1024];
    loop {
        let mut stream = TcpStream::connect(&addr).unwrap();
        let msg = "WaySLOG comming".as_bytes();
//        避免发送数据太快而刷屏
        thread::sleep_ms(100);
        let rcount = try!(stream.write(&msg));
        let _ = try!(stream.read(&mut buf));
        println!("{}", &buf);
        buf.clear();
    }

    Ok(())
}



