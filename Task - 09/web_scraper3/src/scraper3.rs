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
    let c24hchange_selector=scraper::Selector::parse("td.css-1b7j986").unwrap();
    let c24hvolume_selector=scraper::Selector::parse("td.css-1nh9lk8").unwrap();
    let market_selector=scraper::Selector::parse("td.css-1nh9lk8+td").unwrap();
    
    for element in document.select(&row_selector){
        let crypto_element=element.select(&crypto_selector).next().unwrap();
        let crypto = crypto_element.text().collect::<String>();
        let price_element=element.select(&price_selector).next().unwrap();
        let price=price_element.text().collect::<String>();
        let c24hchange_element=element.select(&c24hchange_selector).next().unwrap();
        let c24hchange=c24hchange_element.text().collect::<String>();
        let c24hvolume_element=element.select(&c24hvolume_selector).next().unwrap();
        let c24hvolume=c24hvolume_element.text().collect::<String>();
        let market_element=element.select(&market_selector).next().unwrap();
        let market=market_element.text().collect::<String>();
        
        wtr.write_record([&crypto, &price, &c24hchange, &c24hvolume, &market]).expect("wtr.write");
     wtr.flush().expect("wtr.flush");
     }
        

     println!("fin.")
}

