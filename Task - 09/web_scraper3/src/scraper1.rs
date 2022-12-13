extern crate reqwest;
extern crate select;
extern crate scraper;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};


fn main() {
    crypto("https://crypto.com/price");
}

fn crypto(url: &str) {

    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());
    let mut wtr=csv::Writer::from_path("data.csv").unwrap();
    wtr.write_record(&["Rank", "story", "price", "change", "volume"]).unwrap();

    let document = Document::from_read(resp).unwrap();


    for node in document.find(Class("css-1cxc880")) {

        let rank = node.find(Class("css-w6jew4")).next().unwrap();

        let price = node.find(Class("css-b1ilzc")).next().unwrap();
        let change = node.find(Class("css-1b7j986")).next().unwrap();
        let volume = node.find(Class("css-1nh9lk8")).next().unwrap();
        let market = node.find(Class("css-1nh9lk8+td")).next();
       
        
        let name = node.find(Class("css-ttxvk0").descendant(Name("p")))
            .next()
            .unwrap()
            .text();
            
        //println!("\n | {} | {} | {} | {} | {} | {:?}\n", rank.text(), name, price.text(), change.text(), volume.text(), market);
        
    for element in document.select(&row_selector) {
        let rank_element=element.select(&rank).next().unwrap();
        let RANK = rank_element.text().collect::<String>();
        
        let story_element=element.select(&story).next().unwrap();
        let STORY = story_element.text().collect::<String>();
        
        let price_element=element.select(&price).next().unwrap();
        let PRICE = price_element.text().collect::<String>();
        
        let change_element=element.select(&change).next().unwrap();
        let CHANGE = change_element.text().collect::<String>();

        let volume_element=element.select(&volume).next().unwrap();
        let VOLUME = volume_element.text().collect::<String>();
        
        wtr.write_record([&RANK, &STORY, &PRICE, &CHANGE, &VOLUME]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
     
     println!("fin.")
}

       
      
    }

