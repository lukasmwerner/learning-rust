use redis::{Commands, RedisResult};
use rocket::routes;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct ResultBundle {
    commandName: String,
    parameters: String,
}

#[derive(Serialize, Deserialize)]
struct WordsResult {
    Headword: String,
    Definition: String,
    Id: u32,
}

#[derive(Serialize)]
struct Context {
    query: String,
    results: Vec<WordsResult>,
}

async fn get_results(word: &str) -> Result<Vec<WordsResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = format!(
        "{{\"commandName\":\"cmd_autocomplete\",\"parameters\":\"{}\"}}",
        word
    );
    let resp = client
        .post("https://www.shakespeareswords.com/ajax/AjaxResponder.aspx")
        .body(body)
        .send()
        .await;
    if resp.is_err() {
        return Err(Box::new(resp.err().unwrap()));
    }
    let resp = resp.unwrap();
    let body = resp.text().await?;
    let result_bundle: ResultBundle = serde_json::from_str(&body)?;
    let results: Vec<WordsResult> = serde_json::from_str(result_bundle.parameters.as_str())?;
    Ok(results)
}

async fn memo_results(word: &str) -> Vec<WordsResult> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let result : RedisResult<String> = con.get(&word);
    match result {
        Ok(value) => {
            let results: Vec<WordsResult> = serde_json::from_str(&value).unwrap();
            return results;
        }
        Err(_) => {
            let res = get_results(word).await;
            if res.is_err() {
                return Vec::new();
            }
            let res = res.unwrap();
            let value = serde_json::to_string(&res).unwrap();
            let _ = con.set::<String, String, String>(word.to_string(), value);
            return res;
        },
    }
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        Context {
            query: "".to_string(),
            results: vec![],
        },
    )
}

#[get("/?<query>")]
async fn search(query: &str) -> Template {
    //let results = get_results(query).await;
    let results = memo_results(query).await;

    /*if results.is_err() {
        return Template::render(
            "index",
            Context {
                query: format!(
                    "Encountered error {} on query {}",
                    results.err().unwrap().to_string(),
                    query
                ),
                results: vec![],
            },
        );
    }
    let results = results.unwrap();*/

    Template::render(
        "index",
        Context {
            query: query.to_string(),
            results: results,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/", routes![search])
}
