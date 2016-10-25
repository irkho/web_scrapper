extern crate hyper;
extern crate scraper;

use std::io::{self, Read};
use std::string::String;
use hyper::Client;
use hyper::header::Connection;
use scraper::{Html, Selector};

struct Page {
    status:  hyper::status::StatusCode,
    details: String,
    title:   String,
    body:    String,
}

fn grab_page(url: &str) -> Page {
    let client = Client::new();
    let mut res = client.get(url)
        .header(Connection::close()).send().unwrap();

    let mut result = String::new();
    res.read_to_string(&mut result).unwrap();
    
    let fragment = Html::parse_fragment(&result);

    let selector_title = Selector::parse("title").unwrap();
    let get_title = &fragment.select(&selector_title).next().unwrap();
    let title_text = get_title.text().collect::<Vec<_>>();
    let title: String = title_text.into_iter().collect();
    
    let mut body_vec = Vec::new();
    let selector_p = Selector::parse("p").unwrap();
    for node in fragment.select(&selector_p) {
        let p_text = node.text().collect::<Vec<_>>();
        let body: String = p_text.into_iter().collect();
        println!("{:?}", &body);
        body_vec.push(body)
    }
    
    //let body: String = body_vec.into_iter().collect();
    body_vec.as_mut_slice();
    let body: String = body_vec.join("");
    let body_clean = body.replace("\n", ".");

    Page {
        status:  res.status,
        details: result,
        title:   title,
        body:    body_clean,
    }  
}

fn main() {
    println!("HTML Parser start");
    
    let mut input_url = String::new();
    io::stdin().read_line(&mut input_url)
        .expect("Failed to read the line");
    
    let result = grab_page(&input_url);
    
    println!("### Check response code ###\n{}\n", result.status);
    println!("### Check page title ###\n{}\n", result.title);
    println!("### Check page body ###\n{}\n", result.body);
    println!("### Check complete page ###\n{}\n", result.details);
}
