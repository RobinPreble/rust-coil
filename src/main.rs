use std::{env, io};
use scraper::{Html, Selector};

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
    //response.matches(is_link());
    Ok(())
}

// fn is_link(text: String) -> bool {
//     if text.starts_with("http") && text.ends_with("</a>") {
//         return true
//     } else {
//         false
//     }
// }

