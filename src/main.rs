use reqwest::Error;
pub mod day_one;
pub mod read_input;

#[tokio::main]
async fn main() -> Result<(), Error> {
    day_one::day_one().await;

    Ok(())
}
