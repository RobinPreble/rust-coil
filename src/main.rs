use std::env;
use scraper::{Html, Selector};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the url as an argument");
    }
    
    match get_raw_html(&args[1]) {
        Ok(raw_html) => {
            println!("{}", raw_html);
            println!("Links: {:?}", get_links(&raw_html));
            println!("Images: {:?}", get_images(&raw_html));
        },
        Err(e) => {
            println!("{}", e);
        }
    }    
}
// Takes a url and returns the html from that page 
fn get_raw_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?.text()?;
    Ok(response)
}
// Takes a string containing html and returns a vector of boxes containing all the links
// contained in the html 
fn get_links(raw_html: &String) -> Vec<Box<String>> {
    let document = Html::parse_document(raw_html);
    let link_selector = Selector::parse("a").unwrap();  
    let mut links: Vec<Box<String>> = Vec::new();

    for lnk in document.select(&link_selector) {
        let href = lnk.value().attr("href").unwrap_or("invalid link").to_owned();
        if href.starts_with("http"){
            links.push(Box::new(href));
        }
    }
    links
}

// Takes a string containing html and returns a vector of boxes containing all the images
// contained in the html 
fn get_images(raw_html: &String) -> Vec<Box<String>> {
    let document = Html::parse_document(raw_html);
    let image_selector = Selector::parse("img").unwrap();
    let mut images: Vec<Box<String>> = Vec::new();

    for img in document.select(&image_selector) {
        let src = img.value().attr("src").unwrap_or("invalid image").to_owned();
        images.push(Box::new(src));
    }
    images
}