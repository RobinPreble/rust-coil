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

    let document = Html::parse_document(&response);
    let article_selector = Selector::parse("a").unwrap();

    let mut links: Vec<&str> = Vec::new();

    for element in document.select(&article_selector) {
        let href = element.value().attr("href").unwrap_or("invalid link");
        if href.starts_with("http"){
            links.push(&href);
        }
    }
    println!("Links: {:?}", links);

    
    Ok(())
}



