use libnasu::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        println!("{}", e.to_string());
    };
}
