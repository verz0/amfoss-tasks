extern crate reqwest;
extern crate scraper;

use std::env;
use scraper::{Html, Selector};


fn main(){
    scrape_cryptocurrency("https://crypto.com/price");
    env::set_var("RUST_BACKTRACE", "1");
}

fn scrape_cryptocurrency(url:&str){

    let req = reqwest::blocking::get(url).unwrap();
    assert!(req.status().is_success());
    let mut wtr=csv::Writer::from_path("data.csv").unwrap();
    wtr.write_record(&["cryptocurrency", "change", "number", "price", "volume","market"]).unwrap();
    let doc_body = Html::parse_document(&req.text().unwrap());
    
    let cryptocurrency = Selector::parse("p.chakra-text.css-rkws3").unwrap();
    let change = Selector::parse("p.chakra-text.css-yyku61").unwrap();
    let number = Selector::parse("td.css-w6jew4").unwrap();
    let price = Selector::parse("div.css-b1ilzc").unwrap();
    let volume = Selector::parse("td.css-1nh9lk8").unwrap();
    let market = Selector::parse("td.css-1nh9lk8+td").unwrap();
    
    for cryptocurrency in doc_body.select(&cryptocurrency){
        let cryptocurrencies = cryptocurrency.text().collect::<String>();
        wtr.write_record([&cryptocurrencies]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
        }
    
    for change in doc_body.select(&change){
        let changes = change.text().collect::<String>();
         wtr.write_record([&changes]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
        }
        
    for number in doc_body.select(&number){
        let numbers = number.text().collect::<String>();
         wtr.write_record([&numbers]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
        }
        
    for price in doc_body.select(&price){
        let prices = price.text().collect::<String>();
         wtr.write_record([&prices]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
        }
        
    for volume in doc_body.select(&volume){
        let volumes = volume.text().collect::<String>();
         wtr.write_record([&volumes]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
        }
    
    for market in doc_body.select(&market){
        let markets = market.text().collect::<String>();
         wtr.write_record([&markets]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
        }    
    println!("fin.")
}


