use mongodb::bson::{self, DateTime};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use mongodb::bson::serde_helpers::{serialize_hex_string_as_object_id, deserialize_hex_string_from_object_id, deserialize_rfc3339_string_from_bson_datetime};
use crate::models::job::bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    #[serde(rename = "_id", serialize_with = "serialize_hex_string_as_object_id", deserialize_with = "deserialize_hex_string_from_object_id")]
    pub id: String,
    pub title: String,
    pub location: String,
    pub company: String,
    pub email: String,
    pub description: String,
    pub salary: String,
    pub hiring: bool,
    pub period: String,
    pub contract: String,
    pub requirements: Vec<String>,
    pub skills_required: Vec<String>,
    pub image_url: String,
    pub agent_id: ObjectId,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string",)]
    pub created_at: DateTime,
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string",)]
    pub updated_at: DateTime,
}

impl From<Job> for Document {
    fn from(job: Job) -> Self {
        bson::to_document(&job).unwrap()
    }
}
