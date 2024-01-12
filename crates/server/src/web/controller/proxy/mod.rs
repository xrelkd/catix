use axum::{body::Body, http::Request, response::Response, Extension};
use http::StatusCode;

pub async fn execute(
    Extension(client): Extension<reqwest::Client>,
    Extension(upstream_servers): Extension<Vec<http::Uri>>,
    req: Request<Body>,
) -> Result<Response, StatusCode> {
    let method =
        reqwest::Method::try_from(req.method().as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;
    let path = req.uri().path().trim_start_matches('/');

    for upstream_server in &upstream_servers {
        let uri = format!("{upstream_server}{path}");
        tracing::info!("Try to fetch {uri}");

        let upstream_resp = match client.request(method.clone(), &uri).send().await {
            Ok(resp) => resp,
            Err(_err) => continue,
        };

        if upstream_resp.status().as_u16() != StatusCode::OK {
            continue;
        }

        let mut response_builder = Response::builder();
        for (name, value) in upstream_resp.headers() {
            let name = http::HeaderName::from_bytes(name.as_str().as_bytes());
            let value = http::HeaderValue::from_bytes(value.as_bytes());
            response_builder = match (name, value) {
                (Ok(name), Ok(value)) => response_builder.header(name, value),
                _ => continue,
            };
        }

        return response_builder
            .body(Body::from_stream(upstream_resp.bytes_stream()))
            .map_err(|_| StatusCode::NOT_FOUND);
    }

    tracing::warn!("{path} not found");
    Err(StatusCode::NOT_FOUND)
}
