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
    console_log!(
        "{} {}, located at: {:?}, within: {}",
        req.method().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );

    if !matches!(req.method(), Method::Post) {
        return Response::error("Method Not Allowed", 405);
    }

    let Ok(json) = req.json::<JsonRequest>().await else {
        return Response::error("Bad Request", 400);
    };
    let error_fetch = format!(
        "Cannot fetch the URL {}. Please check corret GOOGLE DOCS PUBLISHED URL.",
        json.url
    );
    let content = reqwest::get(json.url)
        .await
        .or(Err(error_fetch.clone()))?
        .text()
        .await
        .or(Err(error_fetch))?;

    let Some(result) = parse(&content) else {
        return Response::error(vec![
            "Not found doc-content.",
            "Please check that the link is for a Google Docs published to the Web page.",
        ].join(" "), 400);
    };

    Response::ok(result)
}
