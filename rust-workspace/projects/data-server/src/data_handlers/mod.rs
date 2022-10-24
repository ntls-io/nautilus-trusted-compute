use actix_web::{web, HttpResponse, Responder};
use bson::{doc, Bson};
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client};
use std::sync::Mutex;
use chrono::prelude::*;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct UserData {
    pub id: String,
}

#[derive(Deserialize)]
pub struct NewDataPool {
    pub id: String,
    pub poolName: String, //User defined pool name
    pub sealedData: String, //Update data format to match sealed data
}

// Mongo DB (CosmosDB) Name
const MONGO_DB: &'static str = "ntc-data";
// Mongo Collection Name
const MONGO_COLLECTION: &'static str = "datapools";

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/data")
            .route(web::get().to(get_data))
            .route(web::post().to(add_data)),
    );
}

// This function accepts application data
async fn get_data(data: web::Data<Mutex<Client>>, existingUser: web::Json<UserData>) -> impl Responder {
    let data_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);

    let userId = &existingUser.id;
    let filter = doc! {"userId": userId};
    let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
    let mut cursor = data_collection.find(filter, find_options).await.unwrap();

    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}

async fn add_data(data: web::Data<Mutex<Client>>, new_pool: web::Json<NewDataPool>) -> impl Responder {
    let data_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION);

    match data_collection.insert_one(doc! {"userId": &new_pool.id, "poolName": &new_pool.poolName, "sealedData": &new_pool.sealedData, "createdOn": Bson::DateTime(Utc::now())}, None).await {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New document inserted with id {}", new_id);   
            }
            return HttpResponse::Created().json(db_result.inserted_id)
        }
        Err(err) =>
        {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish()
        }
    }
}
