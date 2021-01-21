use libnasu::tasks::adapters::from_file;

#[tokio::main]
async fn main() {
    println!("{:#?}", from_file("nasu.json").unwrap());
}
