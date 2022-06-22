use csv::StringRecord;
use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
mod filter_domains;
use filter_domains::{filter_domains};
use actix_cors::Cors;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

struct DomainData {
    domains: Vec<String>
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[get("/domain")]
async fn domain(data: web::Data<DomainData>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let rand_num = rand::thread_rng().gen_range(0..data.domains.len());
    Ok(format!("{}", data.domains[rand_num]))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    // client_options.app_name = Some("My App".to_string());
    // let db_client = Client::with_options(client_options)?;
    // let db = db_client.database("random-site");
    // let collection = db.collection::<Domain>("domains");
    // let domains = collection.find(doc!{}, None).await?.try_collect().await.unwrap_or_else(|_| vec![]);
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut domains: Vec<String> = vec![];
    let mut rdr = csv::Reader::from_reader(file);

    let string_header = StringRecord::from(vec!["domain"]);
    for record in rdr.records() {
        domains.push(record?.deserialize(Some(&string_header))?);
    }

    let data = web::Data::new(DomainData{ domains: domains });

    //filter_domains().await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .service(domain)
    })
    .bind(("192.168.1.244", 8080))?
    .run()
    .await?;

    Ok(())
}