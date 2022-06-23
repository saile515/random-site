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
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}