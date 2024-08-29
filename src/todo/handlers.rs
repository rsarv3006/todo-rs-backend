use actix_web::{http::Error, HttpResponse};

pub async fn get_todo_list() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello World!"))
}
