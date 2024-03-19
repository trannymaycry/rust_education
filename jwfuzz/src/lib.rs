// library describe base functions for commit
// requests and parse json body response and find out
// true statements responses in dependence of user input desire
// #![deny(warnings)]
// #![warn(rust_2018_idioms)]
// use hyper::{Client, http, Request, Uri};
// use hyper::body::{HttpBody as _, Buf};
// use tokio::io::{stdout, AsyncWriteExt as _};
// // use serde::Deserialize;
// // use crate::http::Response;
//
// pub async fn make_request() {
//     let client = Client::new();
//     let url: Uri = "http://ident.me/".parse().unwrap();
//     match client.get(url).await {
//         Ok(res) => println!("Body is: {:?}", res.body()),
//         Err(err) => println!("Error: {}", err),
//     }
// }
//
// pub async fn make_request_get(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> { // Result<(), Box<dyn std::error::Error + Send + Sync>>
//     let client = Client::new();
//     let tovar = url.parse()?;
//     let mut request = Request::builder()
//         .uri(url)
//         .header("User-Agent", "Mozilla/5.0 (Windows NT; Win32; x86) Gecko/20100101 Firefox/95.0");
//     let mut resp = client.get(tovar).await?;
//
//     while let Some(chunk) = resp.body_mut().data().await {
//         println!("chunk: {:?}", chunk);
//         stdout().write_all(&chunk?).await?;
//     }
//     Ok(())
// }
//
// pub async fn fetch_json(url: hyper::Uri) -> Result<(), ()> {
//     let client = Client::new();
//     // Fetch the url...
//     let res = client.get(url).await?;
//     // asynchronously aggregate the chunks of the body
//     let body = hyper::body::aggregate(res).await?;
//     // try to parse as json with serde_json
//     let users = serde_json::from_reader(body.reader())?;
//     println!("body: {:?}", users);
//     Ok(users)
// }
use hyper::body::Buf;
use hyper::{Client, Request, Uri};
use serde::Deserialize;
use std::collections::HashMap;

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn hyper_fetch_json(url: hyper::Uri) -> Result<Vec<User>> { // Result<Vec<User>>
    let client = Client::new();
    // let client = Request::builder()
    //     .method("GET")
    //     .uri(url)
    //     .header("User-Agent", "Mozilla/3.0 (Windows NT; Win32; x86) Gecko/20100101 Firefox/93.0")
    //     .body(())
    //     .unwrap();

    // Fetch the url...
    let res = client.get(url).await?;
    // asynchronously aggregate the chunks of the body
    let body = hyper::body::aggregate(res).await?;
    // try to parse as json with serde_json
    let users = serde_json::from_reader(body.reader())?;
    Ok(users)
}

pub async fn reqwest_get(url: &str) -> Result<()> {
    let client = reqwest::Client::builder()
        .build()?;

    let res = client
        .get(url)
        .header("User-Agent", "Mozilla/3.0 (Windows NT; Win32; x86) Gecko/20100101 Firefox/93.0")
        .send()
        .await?;
    // let data = res.json("test")
    //     .await?;
    // println!("{:?}", data);
   let data = res
       .json::<HashMap<String, String>>()
       .await?;
    println!("{:?}", data);
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub phone: String,
    #[allow(unused)]
    pub email: String,
    // pub address: Address,
}

pub struct Address {
    city: String,
}