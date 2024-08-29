mod todo;

use actix_web::{web, App, HttpServer};
use todo::setup_todo_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").configure(setup_todo_routes)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
