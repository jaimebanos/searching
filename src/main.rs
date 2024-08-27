mod items;
mod service;
mod status;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

const PATH: &str = "images/";

fn main() {
    // Status controler
    println!("Starting program");

    let mut status = status::StatusProccess::new();

    let new_items: HashSet<items::Item> = items::Item::new("content.json");
    let mut record_items: HashSet<items::Item> = items::Item::new("record.json");
    let service = service::Service::new();

    // Para limpiar los repetidos
    let news_items: HashSet<items::Item> = new_items
        .into_iter()
        .filter(|x| !record_items.contains(x))
        .collect();

    // Los pasa a un vector para poder mutarlos
    let mut news_items: Vec<items::Item> = news_items.into_iter().collect::<Vec<_>>();

    for item in news_items.iter_mut() {
        let website_clean = item
            .website
            .clone()
            .replace("https://", "")
            .replace("http://", "");

        let name = PATH.to_owned() + &website_clean.clone().replace(".com", "") + ".png";

        match service.get(website_clean) {
            Ok(response) => {
                let mut file = File::create(name.clone()).expect("failed to create file");
                file.write_all(&response).expect("failed to write file");
                item.logo = Some(name.clone());
                record_items.insert(item.clone());

                status.update_count_download();
            }
            Err(_) => {
                println!("failed to get image");
                status.update_count_errors();
            }
        }
    }

    let file = File::create("record.json").expect("failed to create file");
    serde_json::to_writer(file, &record_items).expect("failed to write file");

    status.update("Done".to_string());
    println!("{}", status.read_to_string())
}
