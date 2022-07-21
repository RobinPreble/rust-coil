use std::{
    io::{self, Write, Read},
    net::TcpStream,
};
fn main() {
    match main_inner() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main_inner() -> io::Result<()> {
    let mut stream = TcpStream::connect("www.yahoo.com:80")?;
    let bytes_written = stream.write(b"GET / HTTP/1.1\r\n\r\n")?;
    println!("Wrote {} bytes", bytes_written);
    stream.flush()?;
    let mut response: Vec<u8> = vec![]; 
    let mut buffer = [0u8; 256]; 
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        response.extend(buffer.iter());
    }
    println!("Read {} bytes", response.len());
    let response = String::from_utf8(response).unwrap();
    println!("{}", response);
    Ok(())
}