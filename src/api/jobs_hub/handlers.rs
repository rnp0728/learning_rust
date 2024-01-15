use std::str::FromStr;

use crate::models::job::Job;
use actix_web::{get, post, put, web, HttpResponse};
use futures::stream::StreamExt;

use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, FindOneOptions, FindOptions},
    Client,
};

#[get("/")]
pub async fn get_jobs(data: web::Data<Client>) -> HttpResponse {
    let jobs_collection = data.database("jobshubdb").collection::<Job>("jobs");
    let find_options = FindOptions::default();
    let cursor = jobs_collection.find(doc! {}, find_options).await;

    match cursor {
        Ok(mut cursor) => {
              // Collecting items from the cursor into a Vec<Job>
            let mut jobs: Vec<Job> = Vec::new();

            // Iterate over the cursor and collect items
            while let Some(item_result) = cursor.next().await {
                println!("{:?}", item_result);
                match item_result {
                    Ok(item) => jobs.push(item),
                    Err(err) => {
                        return HttpResponse::InternalServerError()
                            .body(format!("Error fetching item from cursor: {}", err));
                    }
                }
            }

            HttpResponse::Ok().json(jobs)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

#[get("/{job_id}")]
pub async fn get_job_by_id(data: web::Data<Client>, path: web::Path<String>) -> HttpResponse {
    let id = path.into_inner();
    let object_id = match ObjectId::from_str(&id) {
        Ok(object_id) => object_id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid job ID format");
        }
    };

    let jobs_collection = data.database("jobshubdb").collection::<Job>("jobs");
    let filter = doc! { "_id": object_id };

    let find_options = FindOneOptions::default();
    let job_result = jobs_collection.find_one(filter, find_options).await;

    match job_result {
        Ok(Some(job)) => HttpResponse::Ok().json(job),
        Ok(None) => HttpResponse::NotFound().body("Job not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
#[put("/{job_id}")]
pub async fn update_job_by_id(
    data: web::Data<Client>,
    path: web::Path<String>,
    new_job: web::Json<Job>,
) -> HttpResponse {
    let id = path.into_inner();
    let object_id = match ObjectId::from_str(&id) {
        Ok(object_id) => object_id,
        Err(_) => {
            return HttpResponse::BadRequest().body("Invalid job ID format");
        }
    };

    let jobs_collection = data.database("jobshubdb").collection::<Job>("jobs");
    let filter = doc! { "_id": object_id };

    // Convert new_job to BSON document explicitly
    let new_job_document: Document = match bson::to_document(&new_job.into_inner()) {
        Ok(doc) => doc,
        Err(err) => {
            return HttpResponse::BadRequest()
                .body(format!("Failed to convert Job to Document: {}", err));
        }
    };

    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    let update_result = jobs_collection
        .find_one_and_update(filter, doc! {"$set": new_job_document}, update_options)
        .await;

    match update_result {
        Ok(Some(updated_job)) => HttpResponse::Ok().json(updated_job),
        Ok(None) => HttpResponse::NotFound().body("Job not found"),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
#[post("/create")]
pub async fn create_job(data: web::Data<Client>, new_job: web::Json<Job>) -> HttpResponse {
    let jobs_collection = data.database("jobshubdb").collection::<Job>("jobs");

    let inserted_result = jobs_collection.insert_one(new_job.into_inner(), None).await;

    match inserted_result {
        Ok(result) => {
            if let Some(inserted_id) = result.inserted_id.as_object_id() {
                HttpResponse::Created().json(inserted_id)
            } else {
                HttpResponse::InternalServerError().body("Failed to retrieve inserted ID")
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
