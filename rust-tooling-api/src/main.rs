use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

//You must register all your modules for it to be visible within your project
mod routes;

use dal_layer::models::details::Details;

use crate::routes::{health_check::*, log_routes::*, myservice_routes::*};
use dal_layer::models::{log_model::*, my_service_model::*, response_model::*};
use dal_layer::repository::db::Database;

#[get("/")]
async fn hello() -> impl Responder {
    let d: Details = Details {
        name: "jide".to_string(),
        age: 3,
    };

    HttpResponse::Ok().body("Hello YouTube!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone()) //register or inject the database obj
            .service(hello)
            .service(health_check)
            .service(create_log)
            .service(get_logs_byservices)
            .service(get_logs_services_by_date_range)
            .service(create_service)
            .service(get_services)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}
