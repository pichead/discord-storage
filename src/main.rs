use std::collections::HashMap;

use crate::service::discord::DiscordServiceTrait;
fn main() {
    println!("Hello, world!");

    for i in 0..10 {
        println!("{}", i);

        let sum5 = if i == 5 { i + 5 } else { 0 };

        println!("{}", sum5);
    }

    let mut me = HashMap::new();
    me.insert("name", "bas");
    me.insert("age", "29");

    const FILE_ID: &str = "12345678901234567890";
    let file_cdn_url = get_new_file_cdn_url(FILE_ID);

    println!("Generated Discord file CDN URL: {}", file_cdn_url);

    println!("{:?}", me);

}
