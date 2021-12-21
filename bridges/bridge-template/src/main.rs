#[tokio::main]
async fn main() {
    let mut times = 0;
    loop {
        times += 1;
        println!("bridge-template Count: {}", times);
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
