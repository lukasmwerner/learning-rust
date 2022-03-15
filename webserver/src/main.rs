use rocket::tokio::time::{sleep, Duration};
use std::string::String;
use redis::Commands;
use rocket::serde::{Deserialize, json::Json};
#[macro_use]
extern crate rocket;
extern crate redis;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/async/sleep/<n>")]
async fn sleepawait(n: u64) -> String {
    sleep(Duration::from_secs(n)).await;
    format!("Slept for {} seconds", n)
}

#[get("/redis/<key>")]
fn redisget(key: String) -> String {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    match con.get(&key) {
        Ok(value) => value,
        Err(e) => format!("Error 404: {} not found \n {}", &key, e),
    }
}

#[derive(Deserialize)]
struct SetRedis {
    key: String,
    value: String,
}

#[post("/redis",  data = "<data>")]
fn redisset(data: Json<SetRedis>) -> String {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let _ : () = con.set(&data.key, &data.value).unwrap();
    format!("{} set to {}", &data.key, &data.value)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![sleepawait])
        .mount("/", routes![redisget])
        .mount("/", routes![redisset])
}
