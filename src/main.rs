use std::{
    io::{self, Write, Read},
    net::TcpStream,
    env
};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Please provide the address and port as arguments");
    }
    match main_inner(&args[1], &args[2]) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main_inner(add: &str, port: &str) -> io::Result<()> {
    // let mut stream = TcpStream::connect("www.yahoo.com:80")?; 
    //let mut stream = TcpStream::connect("captive.apple.com:80")?; 
    let mut stream = TcpStream::connect(add.to_string() + ":" + port)?; 

    let response = stream.write(b"GET / HTTP/1.1\r\n\r\n"); 
    let mut recieved: Vec<u8> = vec![]; 
    let mut buffer = [0u8; 256];

    loop {
        let bytes_read = stream.read(&mut buffer)?; 
        recieved.extend(buffer); 
        buffer = [0u8; 256]; //reset buffer
        if bytes_read == 0 {
            break
        }
    }
    let result = String::from_utf8(recieved).unwrap();
    println!("{}", result);
    Ok(())
}