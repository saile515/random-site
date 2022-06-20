use mongodb::{Client, options::ClientOptions};	
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use actix_web::{get, web, App, HttpServer, Responder};
use rand::Rng;
use futures::TryStreamExt;
mod filter_domains;
use filter_domains::{filter_domains};
use actix_cors::Cors;

#[derive(Debug, Serialize, Deserialize)]
struct Domain {
    domain: String
}

struct MongoData {
    collection: mongodb::Collection<Domain>,
    domains: Vec<Domain>
}

#[get("/domain")]
async fn domain(data: web::Data<MongoData>) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let rand_num: u64 = rand::thread_rng().gen_range(0..data.collection.estimated_document_count(None).await?);
    Ok(format!("{}", data.domains[rand_num as usize].domain))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("My App".to_string());
    let db_client = Client::with_options(client_options)?;
    let db = db_client.database("random-site");
    let collection = db.collection::<Domain>("domains");
    let domains = collection.find(doc!{}, None).await?.try_collect().await.unwrap_or_else(|_| vec![]);

    let data = web::Data::new(MongoData{ collection: collection, domains: domains });

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