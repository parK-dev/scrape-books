use dotenvy::dotenv;
use reqwest::Client;
use scraper::{Html, Selector};
use std::env;

pub mod models;
pub mod schema;

#[tokio::main]
async fn main() {
    let _client = Client::new();
    let body = reqwest::get("http://books.toscrape.com/")
        .await
        .unwrap()
        .text()
        .await
        .expect("Failed to get body!");
    let html = Html::parse_fragment(&body);

    // Selectors
    let book_title_selector = Selector::parse("h3 > a[title]").unwrap();
    let book_price_selector = Selector::parse(".price_color").unwrap();

    // Collections
    let book_titles: Vec<_> = html.select(&book_title_selector).collect();
    let book_prices: Vec<_> = html.select(&book_price_selector).collect();

    use self::schema::books::dsl::*;
    let mut booklist = Vec::new();
    let connection = &mut establish_connection();
    // Iterate over the two collections simultaneously
    for (title_element, price_element) in book_titles.iter().zip(book_prices.iter()) {
        let book_title = title_element
            .value()
            .attr("title")
            .unwrap_or("No title found!");

        // Assuming price_color class contains the price within the element
        let book_price = price_element.inner_html();

        // remove the pound sign
        let book_price = book_price.chars().skip(1).collect::<String>();

        println!("Title: {}, Price: {}", book_title, book_price);

        let book = NewBook {
            title: book_title,
            price: book_price.parse::<f64>().unwrap_or_default(),
            in_stock: true,
        };

        booklist.push(book);
    }

    upsert_books(connection, booklist);

    let results = books
        .filter(in_stock.eq(true))
        .limit(5)
        .load::<models::Book>(connection)
        .expect("Error loading books");

    println!("Displaying {} books", results.len());
    for book in results {
        println!("{}", book.title);
        println!("-----------\n");
        println!("£{}", book.price);
        println!("-----------\n");
        println!("{}", book.in_stock);
    }
}

use diesel::prelude::*;
use diesel::{pg::PgConnection, upsert::excluded};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{Book, NewBook};

pub fn upsert_book(
    conn: &mut PgConnection,
    title: &str,
    price: f64,
    in_stock: bool,
) -> Result<Book, diesel::result::Error> {
    use crate::schema::books;

    let new_book = NewBook {
        title,
        price,
        in_stock,
    };

    diesel::insert_into(books::table)
        .values(&new_book)
        .on_conflict(books::title)
        .do_update()
        .set(&new_book)
        .get_result(conn)
}

pub fn upsert_books(conn: &mut PgConnection, booklist: Vec<NewBook>) -> Vec<Book> {
    use crate::schema::books::dsl::*;

    diesel::insert_into(books)
        .values(&booklist)
        .on_conflict(title)
        .do_update()
        .set((price.eq(excluded(price)), in_stock.eq(excluded(in_stock))))
        .get_results(conn)
        .expect("Error saving new books")
}
