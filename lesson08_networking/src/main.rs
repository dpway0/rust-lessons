use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, header};
use hyper_util::rt::TokioIo;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Todo {
    title: String,
}

#[derive(Serialize)]
struct Message {
    hello: String,
    number: u32,
}

// Build a JSON response with the given status
fn json_response<T: serde::Serialize>(val: &T, status: StatusCode) -> Response<Full<Bytes>> {
    let body = serde_json::to_vec(val).unwrap_or_else(|_| b"{}".to_vec());
    let mut resp = Response::new(Full::new(Bytes::from(body)));
    *resp.status_mut() = status;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    resp
}

// Build a plain-text response with the given status
fn text_response(txt: &str, status: StatusCode) -> Response<Full<Bytes>> {
    let mut resp = Response::new(Full::new(Bytes::from(txt.to_owned())));
    *resp.status_mut() = status;
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    resp
}

// Hop-by-hop headers should not be forwarded by proxies (RFC 7230 §6.1)
fn is_hop_by_hop(name: &hyper::header::HeaderName) -> bool {
    matches!(
        name.as_str().to_ascii_lowercase().as_str(),
        "connection"
            | "keep-alive"
            | "proxy-authenticate"
            | "proxy-authorization"
            | "te"
            | "trailers"
            | "transfer-encoding"
            | "upgrade"
    )
}

// Minimal query parser for `?key=value`; returns value as-is (not URL-decoded)
fn extract_query_param(uri: &hyper::Uri, key: &str) -> Option<String> {
    uri.query().and_then(|q| {
        for pair in q.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                if k == key {
                    return Some(v.to_string());
                }
            }
        }
        None
    })
}

// Simple GET proxy: fetch `target` via reqwest and mirror status/body/headers
async fn proxy_get(
    client: &reqwest::Client,
    target: &str,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // Basic SSRF guard
    let Ok(url) = reqwest::Url::parse(target) else {
        return Ok(text_response("Invalid url", StatusCode::BAD_REQUEST));
    };
    if url.scheme() != "http" && url.scheme() != "https" {
        return Ok(text_response("Unsupported scheme", StatusCode::BAD_REQUEST));
    }
    let allowed = ["jsonplaceholder.typicode.com", "api.github.com"];
    if !allowed.iter().any(|h| Some(*h) == url.host_str()) {
        return Ok(text_response("Host not allowed", StatusCode::FORBIDDEN));
    }

    // Upstream request
    let res = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[proxy] request error: {e}");
            return Ok(text_response(
                "Upstream fetch failed",
                StatusCode::BAD_GATEWAY,
            ));
        }
    };

    // 👇 Extract what you need BEFORE consuming `res`
    let status = res.status();
    let headers_clone = res.headers().clone(); // HeaderMap implements Clone

    // This consumes `res` (ok now)
    let body_bytes = match res.bytes().await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[proxy] read body error: {e}");
            return Ok(text_response(
                "Upstream read failed",
                StatusCode::BAD_GATEWAY,
            ));
        }
    };

    // Build downstream response
    let mut out = Response::new(Full::new(Bytes::from(body_bytes.to_vec())));
    *out.status_mut() = StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);

    // Copy end-to-end headers only
    for (name, value) in headers_clone.iter() {
        if !is_hop_by_hop(name) {
            out.headers_mut().insert(name.clone(), value.clone());
        }
    }

    Ok(out)
}

// Hyper handler: routes /, /proxy/todo, and /proxy?url=...
async fn handle(
    req: Request<Incoming>,
    client: Client,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Fixed proxy endpoint for a sample JSON
        (&Method::GET, "/proxy/todo") => {
            proxy_get(&client, "https://jsonplaceholder.typicode.com/todos/1").await
        }

        // Dynamic proxy endpoint: /proxy?url=https://host/path
        (&Method::GET, "/proxy") => {
            if let Some(url) = extract_query_param(req.uri(), "url") {
                proxy_get(&client, &url).await
            } else {
                Ok(text_response(
                    "Missing url query param",
                    StatusCode::BAD_REQUEST,
                ))
            }
        }

        // Root: simple JSON response
        (&Method::GET, "/") => {
            let msg = Message {
                hello: "Hi, dp from Hyper JSON".to_string(),
                number: 8,
            };
            Ok(json_response(&msg, StatusCode::OK))
        }

        // 404 for everything else
        _ => Ok(text_response("Not Found", StatusCode::NOT_FOUND)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- Reqwest: fetch JSON and print a field (demo) ----
    let client = Client::new();
    let todo: Todo = client
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    println!("Title: {}", todo.title);

    // ---- Hyper server bootstrap ----
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{addr}");

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let client_cloned = client.clone();

        tokio::spawn(async move {
            let svc = service_fn(move |req| handle(req, client_cloned.clone()));
            if let Err(err) = http1::Builder::new().serve_connection(io, svc).await {
                eprintln!("server error: {err}");
            }
        });
    }
}
