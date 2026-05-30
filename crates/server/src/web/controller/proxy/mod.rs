use std::{path::PathBuf, str::FromStr};

use axum::{Extension, body::Body, http::Request, response::Response};
use http::StatusCode;

pub async fn execute(
    Extension(client): Extension<reqwest::Client>,
    Extension(upstream_servers): Extension<Vec<http::Uri>>,
    req: Request<Body>,
) -> Result<Response, StatusCode> {
    let method =
        reqwest::Method::try_from(req.method().as_str()).map_err(|_| StatusCode::BAD_REQUEST)?;

    let path_and_query = req.uri().path_and_query().filter(|pq| pq.path().starts_with('/'));
    for upstream_server in &upstream_servers {
        if let Some(uri) = derive_uri(upstream_server, path_and_query) {
            tracing::info!("Try to fetch {uri}");

            let upstream_resp = match client.request(method.clone(), uri.to_string()).send().await {
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
    }

    if let Some(path_and_query) = path_and_query {
        tracing::warn!("{path_and_query} not found");
    }
    Err(StatusCode::NOT_FOUND)
}

fn derive_uri(
    upstream_server: &http::Uri,
    path_and_query: Option<&http::uri::PathAndQuery>,
) -> Option<http::Uri> {
    let path_and_query = path_and_query.map_or_else(
        || "/".to_string(),
        |path_and_query| {
            let mut path = PathBuf::from_str(upstream_server.path()).unwrap_or_default();
            path.push(path_and_query.path().trim_start_matches('/'));
            path_and_query.query().map_or_else(
                || format!("{}", path.display()),
                |query| format!("{}?{query}", path.display()),
            )
        },
    );

    http::Uri::builder()
        .scheme(upstream_server.scheme().unwrap_or(&http::uri::Scheme::HTTPS).clone())
        .authority(upstream_server.authority()?.clone())
        .path_and_query(path_and_query)
        .build()
        .ok()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::derive_uri;

    #[test]
    fn test_uri_path_and_query_splits() {
        let cases = [
            (
                "/nar/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo",
                Some("/nar/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"),
                None,
            ),
            (
                "/nar/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo?hash=abc",
                Some("/nar/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"),
                Some("hash=abc"),
            ),
            ("/", Some("/"), None),
            ("/?hash=abc", Some("/"), Some("hash=abc")),
            (
                "//q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo",
                Some("//q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"),
                None,
            ),
            (
                "////q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo?hash=abc",
                Some("////q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"),
                Some("hash=abc"),
            ),
            ("example.com:443", None, None),
        ];
        for (input, expected_path, expected_query) in cases {
            let uri = http::Uri::from_str(input).unwrap();
            let pq = uri.path_and_query();
            let actual = pq.map(|pq| (pq.path(), pq.query()));
            let expected = expected_path.map(|p| (p, expected_query));
            assert_eq!(actual, expected, "input: {input}");
        }
    }

    #[test]
    fn test_uri_path_always_starts_with_slash() {
        let inputs = [
            "/nar/file.narinfo",
            "/nar/file.narinfo?hash=abc",
            "/",
            "/?hash=abc",
            "//double",
            "////quadruple",
            "https://example.com/path",
            "https://example.com/path?q=1",
        ];
        for input in inputs {
            let uri = http::Uri::from_str(input).unwrap();
            let pq = uri.path_and_query();
            if let Some(pq) = pq {
                assert!(
                    pq.path().starts_with('/'),
                    "path should start with '/' for input: {input}"
                );
            }
        }
    }

    #[test]
    fn test_derive_uri() {
        assert_eq!(
            http::Uri::from_str("https://cache.nixos.org/").ok(),
            derive_uri(&http::Uri::from_str("https://cache.nixos.org").unwrap(), None)
        );
        assert_eq!(
            http::Uri::from_str("https://cache.nixos.org/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://cache.nixos.org").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str("/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                        .unwrap()
                )
            )
        );

        // Realistic production scenario from execute():
        // upstream has trailing slash, request path is a standard absolute-path.
        assert_eq!(
            http::Uri::from_str("https://cache.nixos.org/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://cache.nixos.org/").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str("/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                        .unwrap()
                )
            )
        );

        assert_eq!(
            http::Uri::from_str("https://cache.nixos.org/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://cache.nixos.org").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str(
                        "//q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"
                    )
                    .unwrap()
                )
            )
        );

        assert_eq!(
            http::Uri::from_str("https://cache.nixos.org/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://cache.nixos.org").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str(
                        "////q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"
                    )
                    .unwrap()
                )
            )
        );

        assert_eq!(
            http::Uri::from_str("https://mirror.iscas.ac.cn/nix-channels/store/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://mirror.iscas.ac.cn/nix-channels/store/").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str(
                        "////q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"
                    )
                    .unwrap()
                )
            )
        );

        assert_eq!(
            http::Uri::from_str("https://mirror.example.ac/nix-channels/store/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://mirror.example.ac/nix-channels/store").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str(
                        "////q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo"
                    )
                    .unwrap()
                )
            )
        );

        assert_eq!(
            http::Uri::from_str("https://mirror.example.ac/nix-channels/store/q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo?hash=abc")
                .ok(),
            derive_uri(
                &http::Uri::from_str("https://mirror.example.ac/nix-channels/store").unwrap(),
                Some(
                    &http::uri::PathAndQuery::from_str(
                        "////q8qq40xg2grfh9ry1d9x4g7lq4ra7n81.narinfo?hash=abc"
                    )
                    .unwrap()
                )
            )
        );
    }
}
