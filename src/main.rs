use reqwest::Error;
pub mod day_five;
pub mod day_four;
pub mod day_one;
pub mod day_three;
pub mod day_two;
pub mod read_input;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // day_one::day_one().await;
    // day_two::day_two().await;
    // day_three::day_three().await;
    // day_four::day_four().await;
    day_five::execute().await;
    Ok(())
}
