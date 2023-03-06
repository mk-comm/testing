use actix_web::{get,web, App, HttpServer, post, HttpResponse};
use serde::{Deserialize, Serialize};

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
        connection(json.into_inner());
        
        
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


