#![allow(warnings)]
/// really simple http request reflector
use argh::FromArgs;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, HeaderMap, Request, Response, Server};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::os::unix::prelude::OsStringExt;

/// Tokio Rustls server example
#[derive(FromArgs)]
struct Options {
    /// server address
    #[argh(positional)]
    addr: SocketAddr,
}

// mod header_map_serde {
//     use super::HeaderMap;
//     use serde::de::{Deserialize, Deserializer};
//     use serde::ser::Serializer;
//
//     pub fn serialize<S>(map: &HeaderMap, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.collect_map(map.iter().map(|(k, v)| ("a", "b")))
//     }
//
//     // pub fn deserialize<'de, D>(deserializer: D) -> Result<Services, D::Error>
//     // where
//     //     D: Deserializer<'de>,
//     // {
//     //     let mut map = Services::new();
//     //     for service in Vec::<Service>::deserialize(deserializer)? {
//     //         map.insert(service.name.clone(), service);
//     //     }
//     //     Ok(map)
//     // }
// }

type Headers = HashMap<String, Vec<String>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Reflection {
    method: String,
    uri: String,
    version: String,
    headers: Headers,
}

/// just reflect method, uri, version and headers
async fn reflect(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut headers = Headers::new();
    headers.extend(req.headers().keys().into_iter().map(|k| {
        (
            k.to_string(),
            Vec::from_iter(
                req.headers()
                    .get_all(k)
                    .into_iter()
                    .map(|v| v.to_str().unwrap().to_string()),
            ),
        )
    }));

    let mut reflection = Reflection {
        method: req.method().to_string(),
        uri: req.uri().to_string(),
        version: format!("{:?}", &req.version()),
        headers: headers,
    };

    let json = serde_json::to_string_pretty(&reflection).unwrap();
    Ok(Response::builder()
        .header("Content-Type", "application/json") // ContentType::json() not there anymore
        .body(Body::from(json))
        .unwrap())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let options: Options = argh::from_env();
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(reflect)) });

    let server = Server::bind(&options.addr).serve(make_svc);
    println!("Listening on http://{}", options.addr);

    server.await?;

    Ok(())
}
