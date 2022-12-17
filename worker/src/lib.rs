use docs_to_markdown::parse;
use serde::{Deserialize, Serialize};
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRequest {
    url: String,
}

#[event(fetch)]
pub async fn main(mut req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    if !matches!(req.method(), Method::Post) {
        return Response::error("Method Not Allowed", 405);
    }

    let Ok(json) = req.json::<JsonRequest>().await else {
        return Response::error("Bad Request", 400);
    };
    let error_fetch = Response::error(
        format!(
            "Cannot fetch the URL {}. Please check corret GOOGLE DOCS PUBLISHED URL.",
            json.url
        ),
        400,
    );

    let Ok(response) = reqwest::get(json.url).await else { return error_fetch; };
    let Ok(content) = response.text().await else { return error_fetch; };

    match parse(&content) {
        Err(err) => Response::error(err, 400),
        Ok(result) => Response::ok(result),
    }
}
