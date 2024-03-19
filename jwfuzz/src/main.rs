// use tokio::net::TcpStream;
// 
// async fn test_async() {
//     println!("hello from async!");
//     let _socket = TcpStream::connect("192.168.1.50:8080");
//     println!("async TCP complete");
// }
// 
// #[tokio::main]
// async fn main() {
//     // let my_async_call = test_async();
//     // my_async_call.await;
//     // let get_body =  jwfuzz::make_request();
//     // get_body.await;
//     // let get_body_chunk = jwfuzz::make_request_get("http://ident.me/");
//     // get_body_chunk.await;
//     let get_body_json = jwfuzz::fetch_json("http://ident.me/");
//     get_body_json.await;
// }
// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // let url = "http://ident.me/".parse().unwrap();
    // let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();
    let url = "https://httpbin.org/ip";
    // let users = jwfuzz::hyper_fetch_json(url).await?;
    // // print users
    // println!("users: {:#?}", users);
    jwfuzz::reqwest_get(url).await?;
    // print the sum of ids
    // let sum = users.iter().fold(0, |acc, user| acc + user.phone);
    // println!("sum of ids: {}", sum);
    Ok(())
}
