use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Element, Name, Predicate};

// trouver tout les <a> tag est en sortir les href
#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let body = reqwest::get("https://www.rust-lang.org/fr")
        .await?
        .text()
        .await?;

    for node in Document::from(body.as_str()).find(Name("a")) {
        println!("{:#?}", node.attr("href").unwrap());
    }

    Ok(())
}
