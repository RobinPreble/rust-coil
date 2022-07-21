use std::{
    io::{self, Read},
    net::TcpStream,
};
fn main() {
    match main_inner() {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main_inner() -> io::Result<()> {
    let mut stream = TcpStream::connect("www.yahoo.com")?; 
    //let response = "HTTP/1.1 200 OK\r\n\r\n"; 
    let mut recieved: Vec<u8> = vec![]; 
    let mut buffer = [0u8; 256];

    loop {
        let bytes_read = stream.read(&mut buffer)?; 
        recieved.extend(buffer); 

        if bytes_read == 0 {
            break
        }
    }
    println!("{:?}", recieved);
    Ok(())
}