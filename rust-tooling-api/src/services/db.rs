use crate::models::{log_model::Log, log_model::LogLevel, my_service_model::MyService};
use crate::utils::date_helper::Converter;

use actix_web::Error;
use chrono::Utc;
use futures_util::stream::StreamExt;

use mongodb::Collection;
use mongodb::bson::from_document;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{DateTime, doc};
use mongodb::options::CountOptions;
use mongodb::results::InsertOneResult;
use mongodb::results::UpdateResult;
use mongodb::{Client, Cursor};
use mongodb::{IndexModel, options::IndexOptions};
use std::env;
use std::str::FromStr;

pub struct Database {
    log: Collection<Log>,
    myservice: Collection<MyService>,
}

impl Database {
    ///This is going to initialize the database
    pub async fn init() -> Self {
        let uri: String = match env::var("MONGO_URI") {
            Ok(s) => s.to_string(),
            Err(_) => {
                println!("extracting the connection string from here");
                String::from("mongodb://localhost:27017/?directConnection=true")
            }
        };

        let client: Client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rust_log_monitor");

        let myservice: Collection<MyService> = db.collection("myservice");
        let log: Collection<Log> = db.collection("log");

        Database { log, myservice }
    }

    pub async fn ensure_created_at_index(
        collection: &mongodb::Collection<Log>,
    ) -> Result<(), Error> {
        let index = IndexModel::builder()
            .keys(doc! { "created_at": 1 }) // ascending
            .options(IndexOptions::builder().build())
            .build();

        collection
            .create_index(index)
            .await
            .ok()
            .expect("Error creating the index");

        Ok(())
    }

    //insert my service into the database
    pub async fn create_service(&self, myservice: MyService) -> Result<InsertOneResult, Error> {
        let result = self
            .myservice
            .insert_one(myservice)
            .await
            .ok()
            .expect("Error inserting application or microserice");

        Ok(result)
    }

    /// Bulk insert using a single `insert_many` call
    pub async fn insert_services_bulk(&self, services: Vec<MyService>) -> Result<(), Error> {
        if services.is_empty() {
            return Ok(()); // nothing to insert
        }

        // Ordered=false means MongoDB will continue inserting even if one fails (e.g., duplicate key)
        let options = mongodb::options::InsertManyOptions::builder()
            .ordered(false)
            .build();

        self.myservice
            .insert_many(services)
            .await
            .ok()
            .expect("problem inserting bulk service");

        Ok(())
    }

    pub async fn get_services(&self) -> Result<Vec<MyService>, Error> {
        // No filter = return all documents (be careful with large collections!)

        let mut cursor: Cursor<MyService> = self.myservice.find(None).await?;
        let services: Vec<MyService> = cursor.try_collect().await?; // or iterate with try_next

        Ok(services)
    }

    /******************************Log Modules below****************************/
    pub async fn create_log(&self, log: Log) -> Result<InsertOneResult, Error> {
        let result = self
            .log
            .insert_one(log)
            .await
            .ok()
            .expect("Error inserting application or microserice");

        Ok(result)
    }

    /// Bulk insert using a single `insert_many` call
    pub async fn insert_logs_bulk(&self, logs: Vec<Log>) -> Result<(), Error> {
        if logs.is_empty() {
            return Ok(()); // nothing to insert
        }

        // Ordered=false means MongoDB will continue inserting even if one fails (e.g., duplicate key)
        let options = mongodb::options::InsertManyOptions::builder()
            .ordered(false)
            .build();

        self.log
            .insert_many(logs)
            .await
            .ok()
            .expect("error performing bulk log ");

        Ok(())
    }

    pub async fn get_logs_by_service(&self, service_id: &String) -> Result<Vec<Log>, Error> {
        let serviceid = ObjectId::from_str(service_id).expect("Failed to parse service_id");

        let filter = doc! { "my_service_id": serviceid };
        let cursor: Cursor<Log> = self.log.find(filter).await?;
        let items: Vec<Log> = cursor.try_collect().await?;
        Ok(items)
    }

    pub async fn get_logs_service_by_date_range(
        &self,
        service_id: String,
        start: DateTime,
        end: DateTime,
    ) -> Result<Vec<MyService>, Error> {
        let filter = doc! {
            "my_service_id": service_id,
            "created_at": { "$gte": start, "$lt": end }
        };

        let mut cursor: Cursor<MyService> = self
            .myservice
            .find(filter)
            .await
            .ok()
            .expect("error getting logs by id and date range");

        let services: Vec<MyService> = cursor.try_collect().await?;
        Ok(services)
    }

    pub async fn delete_logs_by_date_range(
        &self,
        start: DateTime,
        end: DateTime,
    ) -> Result<u64, Error> {
        let filter = doc! {
            "created_at": { "$gte": start, "$lt": end }
        };

        let result = self
            .log
            .delete_many(filter)
            .await
            .ok()
            .expect("error deleting logs by date");

        Ok(result.deleted_count)
    }

    pub async fn count_by_date_range(
        &self,
        start: DateTime,
        end: DateTime,
    ) -> Result<u64, mongodb::error::Error> {
        let filter = doc! { "created_at": { "$gte": start, "$lt": end } };
        self.log.count_documents(filter).await
    }
}
