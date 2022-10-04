// main.rs
use scraper::{Html, Selector};
fn main() {
    let url = "https://books.toscrape.com/";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().expect("No response body found.");
    let document = Html::parse_document(&body);
    let book_selector = Selector::parse("article.product_pod").expect("Could not create selector.");
    let book_name_selector = Selector::parse("h3 a").expect("Could not create selector.");
    let book_price_selector = Selector::parse(".price_color").expect("Could not create selector.");
    let mut wtr = csv::Writer::from_path("books.csv").expect("Could not create file.");
    wtr.write_record(&["Book Name", "Price", "Link"])
        .expect("Could not write header.");
    for element in document.select(&book_selector) {
        let book_name_element = element
            .select(&book_name_selector)
            .next()
            .expect("Could not select book name.");
        let book_name = book_name_element
            .value()
            .attr("title")
            .expect("Could not find title attribute.");
        let price_element = element
            .select(&book_price_selector)
            .next()
            .expect("Could not find price");
        let price = price_element.text().collect::<String>();
        let book_link_element = element
            .select(&book_name_selector)
            .next()
            .expect("Could not find book link element.");
        let book_link = book_link_element
            .value()
            .attr("href")
            .expect("Could not find href attribute");
        wtr.write_record([book_name, &price, &book_link])
            .expect("Could not create selector.");
    }
    wtr.flush().expect("Could not close file");
    println!("Done");
}