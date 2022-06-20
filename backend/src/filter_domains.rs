use std::time::Duration;

use mongodb::{Client, options::ClientOptions};	
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use futures::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
struct Domain {
    domain: String
}

pub async fn filter_domains() -> Result<(), Box<dyn std::error::Error>> {
    let req_client = reqwest::Client::new();
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.app_name = Some("My App".to_string());
    let db_client = Client::with_options(client_options)?;
    let db = db_client.database("random-site");
    let collection = db.collection::<Domain>("domains");
    let mut domains = collection.find(Some(doc!{}), None).await?;

    let mut i = 0;
    while let Some(domain) = domains.try_next().await? {
        println!("{}", domain.domain);
        let req = check_domain( &format!("http://{}", domain.domain), &req_client).await;

        if !req {
            let del_res = collection.delete_one(doc!{"domain": domain.domain}, None).await?;
            println!("{:?}", del_res);
        }
        i += 1;
        println!("{}", i);
    }
    

    Ok(())
}

async fn check_domain(domain: &str, client: &reqwest::Client) -> bool {
    if let Ok(res) = tokio::time::timeout(Duration::from_secs(2), client.get(domain)
    .send()).await {
        if let Ok(res) = res {
            if res.status().is_success() {
                return true;
            } else {
                return false;
            }
        } 
        return false
    } else {
        return false;
    }
}