# Web Scraping with Rust

[![Oxylabs promo code](https://raw.githubusercontent.com/oxylabs/product-integrations/refs/heads/master/Affiliate-Universal-1090x275.png)](https://oxylabs.io/pages/gitoxy?utm_source=877&utm_medium=affiliate&groupid=877&utm_content=web-scraping-with-rust-github&transaction_id=102f49063ab94276ae8f116d224b67)

<p align="left">
  <a href="https://dashboard.oxylabs.io"><img src="https://img.shields.io/badge/%F0%9F%9A%80_Get_Started-FF6B35?style=for-the-badge" alt="Get started"></a>
  <a href="https://developers.oxylabs.io/products/web-api"><img src="https://img.shields.io/badge/%F0%9F%93%9A_Documentation-4A90E2?style=for-the-badge" alt="Documentation"></a>
  <a href="https://dashboard.oxylabs.io"><img src="https://img.shields.io/badge/%F0%9F%93%8A_Dashboard-2EA44F?style=for-the-badge" alt="Dashboard"></a>
  <a href="https://discord.gg/Pds3gBmKMH"><img src="https://img.shields.io/badge/%F0%9F%92%AC_Discord-5865F2?style=for-the-badge" alt="Discord"></a>
  <a href="https://www.youtube.com/@oxylabs"><img src="https://img.shields.io/badge/%E2%96%B6%EF%B8%8F_YouTube-FF0000?style=for-the-badge" alt="YouTube"></a>
</p>

<a href="https://github.com/topics/go"><img src="https://img.shields.io/static/v1?label=&amp;message=rust&amp;color=brightgreen" style="max-width: 100%;"></a> <a href="https://github.com/topics/web-scraping"><img src="https://img.shields.io/static/v1?label=&amp;message=Web%20Scraping&amp;color=important" style="max-width: 100%;"></a>

  - [Installing and running Rust](#installing-and-running-rust)
    - [Installing Rust on Windows](#installing-rust-on-windows)
    - [Installing Rust on macOS and Linux](#installing-rust-on-macos-and-linux)
  - [Rust web scraper for scraping book data](#rust-web-scraper-for-scraping-book-data)
    - [Setup](#setup)
    - [Making an HTTP request](#making-an-http-request)
    - [Parsing HTML with scraper](#parsing-html-with-scraper)
    - [Writing scraped data to a CSV file](#writing-scraped-data-to-a-csv-file)
  - [Running the code](#running-the-code)

Rust is rapidly gaining attention as a programming language offering performance just as high as C/C++, especially regarding web scraping. However, unlike Python, which is relatively easy to learn, oftentimes at the cost of performance, Rust can be tricky to figure out. 

It doesn't mean that scraping with Rust is not possible or extremely hard. Scraping with Rust can be challenging only if you don't know how to begin.

This article will guide you an overview of the process of writing a fast and efficient Rust web scraper.

For a detailed explanation, [see this blog post](https://oxylabs.io/blog/rust-web-scraping).

## Installing and running Rust

Download `rustup` from https://www.rust-lang.org/tools/install page. For Windows, download RUSTUP-INIT and run it to install Rust.

On macOS and Linux, this page will show you the command to install `rustup`. The command will be similar to the following:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Run this command from the terminal and follow the prompts to install Rust.

## Rust web scraper for scraping book data

### Setup

Open the terminal and run the following command to initialize an empty project:

```shell
$ cargo new book_scraper
```

Now, open the `Cargo.toml` file, and add the following lines:

```toml
[dependencies]
reqwest = {version = "0.11", features = ["blocking"]} 
scraper = "0.13.0"
```

### Making an HTTP request

```rust
// main.rs
fn main() {
    let url = "https://books.toscrape.com/";
    let response = reqwest::blocking::get(url).expect("Could not load url.");
    let body = response.text().unwrap();
    print!("{}",body);
}
```

### Parsing HTML with scraper



Open https://books.toscrape.com/ in Chrome and examine the HTML markup of the web page.

![books to scrape source code](https://oxylabs.io/blog/images/2021/12/book_container-1.png)



```rust
use scraper::{Html, Selector};
// ...
let book_selector = Selector::parse("article.product_pod").unwrap();
Now the selector is ready to be used. Add the following lines to the main function:
for element in document.select(&book_selector) {
	// more code here
} 
```

**Extracting product description**

```rust
for element in document.select(&book_selector) {
    let book_name_element = element.select(&book_name_selector).next().expect("Could not select book name.");
    let book_name = book_name_element.value().attr("title").expect("Could not find title attribute.");
    let price_element = element.select(&book_price_selector).next().expect("Could not find price");
    let price = price_element.text().collect::<String>();
   println!("{:?} - {:?}",book_name, price);
}
```

**Extracting product links**

Within the *for loop*, add the following line:

```rust
let book_link_element = element.select(&book_name_selector).next().expect("Could not find book link element.");
let book_link= book_link_element.value().attr("href").expect("Could not find href attribute");
```

### Writing scraped data to a CSV file

First, add the following to `Cargo.toml` dependencies:

```toml
csv="1.1"
```

Update `main.rs` as follows:

```rust
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
```

## Running the code

Enter the following command to run the code:

```shell
$ cargo run
```

If you wish to find out more about web scraping with Rust, see our [blog post](https://oxylabs.io/blog/rust-web-scraping).
