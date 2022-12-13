extern crate reqwest;
extern crate select;
extern crate scraper;




fn main() {
    let response = reqwest::blocking::get("https://crypto.com/price",).unwrap().text().unwrap();
    let mut wtr=csv::Writer::from_path("data.csv").unwrap();
    wtr.write_record(&["Name", "price", "change", "volume", "market"]).unwrap();
    
    let document=scraper::Html::parse_document(&response);
    let row_selector=scraper::Selector::parse("tr.css-1cxc880").unwrap();
    let crypto_selector = scraper::Selector::parse("p.chakra-text.css-rkws3").unwrap();

    let price_selector=scraper::Selector::parse("p.chakra-text css-13hqrwd").unwrap();
    let c24hc_selector=scraper::Selector::parse("td.css-1b7j986").unwrap();
    let c24hvo_selector=scraper::Selector::parse("td.css-1nh9lk8").unwrap();
    let market_selector=scraper::Selector::parse("td.css-1nh9lk8+td").unwrap();
    
    for element in document.select(&row_selector){
        let crypto_element=element.select(&crypto_selector).next().unwrap();
        let crypto = crypto_element.text().collect::<String>();
        wtr.write_record([&crypto]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
        

        
    for element in document.select(&row_selector){
        let price_element=element.select(&price_selector).next().unwrap();
        let price=price_element.text().collect::<String>();
        wtr.write_record([&price]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
        
    for element in document.select(&row_selector){
        let c24hc_element=element.select(&c24hc_selector).next().unwrap();
        let c24hc=c24hc_element.text().collect::<String>();
        wtr.write_record([&c24hc]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
        
    for element in document.select(&row_selector){
        let c24hvo_element=element.select(&c24hvo_selector).next().unwrap();
        let c24hvo=c24hvo_element.text().collect::<String>();
        wtr.write_record([&c24hvo]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
        
    for element in document.select(&row_selector){
        let market_element=element.select(&market_selector).next().unwrap();
        let market=market_element.text().collect::<String>();
        wtr.write_record([&market]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
        

     println!("fin.")
}

