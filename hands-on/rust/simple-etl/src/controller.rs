use serde_json::Value;
use rocket::serde::json::Json;
use rocket::http::ContentType;
use crate::jsonfn::*;


#[get("/")]
pub fn index() -> &'static str {
    "Welcome to simple-etl!"
}

#[post("/filter", data = "<data>", format = "application/json")]
pub fn filter(data: Json<Value>) -> (ContentType, String) {
    let data_json = data.into_inner();
    let filterd = json_filter(&data_json.as_array().unwrap(), |v| v["age"].as_i64().unwrap() > 25);
    let result = serde_json::to_string(&filterd).unwrap();
    (ContentType::JSON, result)
}

#[post("/map", data = "<data>", format = "application/json")]
pub fn map(data: Json<Value>) -> (ContentType, String) {
    let data_json = data.into_inner();
    let mapped = json_map(&data_json.as_array().unwrap(), |v| v.clone());
    let result = serde_json::to_string(&mapped).unwrap();
    (ContentType::JSON, result)
}

#[post("/fold", data = "<data>", format = "application/json")]
pub fn fold(data: Json<Value>) -> (ContentType, String) {
    let data_json = data.into_inner();
    let folded = json_fold(&data_json.as_array().unwrap(), 0, |acc, v| acc + v["age"].as_i64().unwrap());
    let result = serde_json::to_string(&folded).unwrap();
    (ContentType::Text, result)
}



