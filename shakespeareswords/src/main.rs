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
    let results = get_results(query).await;

    if results.is_err() {
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
    let results = results.unwrap();

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
