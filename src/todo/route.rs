use super::handlers::get_todo_list;
use actix_web::web;

pub fn setup_todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo").service(web::resource("/").route(web::get().to(get_todo_list))),
    );
}
