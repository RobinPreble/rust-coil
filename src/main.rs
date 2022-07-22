use std::{
    io::{self, Write, Read},
    net::{TcpStream, Shutdown},
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

fn main_inner(host: &str, port: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(host.to_string() + ":" + port)?; 
    let header = format!("GET / HTTP/1.1\nUser-Agent: curl/7.77.0\nAccept: */*\nHost: {}:{}\nConnection: close\n\n", host, port);
    let _result = stream.write(&header.as_bytes()).unwrap(); 
    let mut recieved: Vec<u8> = vec![]; 
    let mut buffer = [0u8; 256];

    loop {
        let bytes_read = stream.read(&mut buffer)?; 
        recieved.extend(buffer); 
        buffer = [0u8; 256]; // reset buffer
        if bytes_read == 0 {
            break
        }
    }
    //stream.shutdown(Shutdown::Both).expect("u done fucked up");

    let result = String::from_utf8(recieved).unwrap();
    println!("{}", result);
    Ok(())
}