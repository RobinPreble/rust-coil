use std::{env, io};


//use openssl::ssl::{SslMethod, SslConnectorBuilder, SslConnector};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the address and port as arguments");
    }
    
    match main_inner(&args[1]) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main_inner(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    println!("{}", response);
    Ok(())
}