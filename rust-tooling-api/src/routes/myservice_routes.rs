use actix_web::{
    Error, HttpResponse,
    web::{Data, Json},
};
use actix_web::{get, post};

use dal_layer::{
    models::my_service_model::{MyService, MyServiceView},
    repository::db::Database,
};

#[post("/service")]
pub async fn create_service(db: Data<Database>, request: Json<MyServiceView>) -> HttpResponse {
    match db
        .create_service(
            MyService::try_from(MyServiceView {
                service_id: None,
                name: request.name.clone(),
                description: request.description.clone(),
                onboarded_datetime: request.onboarded_datetime.clone(),
            })
            .expect("Error converting Service request to Serviice entity."),
        )
        .await
    {
        Ok(myservice) => HttpResponse::Ok().json(myservice),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/services")]
pub async fn get_services(db: Data<Database>) -> HttpResponse {
    match db.get_services().await {
        Ok(services) => HttpResponse::Ok().json(services),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
