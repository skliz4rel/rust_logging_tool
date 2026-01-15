use crate::{
    models::log_model::{Log, LogRequest},
    services::db::Database,
    utils::date_helper::Converter,
};
use actix_web::{
    Error, HttpResponse,
    web::{Data, Json, Path},
};
use actix_web::{get, post};
use mongodb::bson::DateTime;

#[post("/log")]
pub async fn create_log(db: Data<Database>, request: Json<LogRequest>) -> HttpResponse {
    match db
        .create_log(
            Log::try_from(LogRequest {
                level: request.level.clone(),
                my_service_id: request.my_service_id.clone(),
                line_content: request.line_content.clone(),
                created_at: request.created_at.clone(),
            })
            .expect("Error converting DogRequest to Dog."),
        )
        .await
    {
        Ok(dog) => HttpResponse::Ok().json(dog),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/logs/{service_id}")]
pub async fn get_logs_byservices(db: Data<Database>, path: Path<(String,)>) -> HttpResponse {
    let service_id: String = path.into_inner().0;
    match db.get_logs_by_service(&service_id).await {
        Ok(services) => HttpResponse::Ok().json(services),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/logs/bydate/{service_id}/{start_date}/{end_date}")]
pub async fn get_logs_services_by_date_range(
    db: Data<Database>,
    path: Path<(String, String, String)>,
) -> HttpResponse {
    let (service_id, start_date, end_date) = &path.into_inner();

    let start_date: DateTime = Converter::convert_str_datetime(&start_date);
    let end_date: DateTime = Converter::convert_str_datetime(&end_date);

    match db
        .get_logs_service_by_date_range(service_id, start_date, end_date)
        .await
    {
        Ok(services) => HttpResponse::Ok().json(services),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
