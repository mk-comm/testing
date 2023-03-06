use actix_web::{get,web, App, HttpServer, post, HttpResponse};

mod structs;
mod actions;

use structs::entry::Entry;
use crate::actions::connection::connection;

#[get("/")]
async fn index() -> String{
    "Hello world!".to_string()
}

#[post("/connect")]
async fn connect(json: web::Json<Entry>) -> HttpResponse {
    tokio::spawn(async move {
        let api = connection(json.into_inner());
        match api {
            Ok(_) => println!("Connected!"),
            Err(error) => {let client = reqwest::Client::new();
            let _res = client.post("https://webhook.site/79c8d1a4-2f91-49a1-9d59-9a41c4f8b8ec")
                .body(error.to_string())
                .send()
                .await;},
        }
        
    });
    
    HttpResponse::Ok().body("Connected!")
    
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(connect)
    })
    .bind("127.0.0.1:8080")?.run().await
}


