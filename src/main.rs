use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let session_cookie = "session=53616c7465645f5f7a93bee72655ac5d5dd0251b80c13d0b1e70c999a360ce05cf6fe049f722f71ff5c7bf86df9ab556e960da9ad0b9e77c0cc9cd1856c144af";
    let client = reqwest::Client::builder().build()?;
    let res = client
        .get("https://adventofcode.com/2022/day/1/input")
        .header("Cookie", session_cookie)
        .send()
        .await?
        .text()
        .await?;
    let calorylist_per_elve: Vec<Vec<i32>> = res
        .split("\n\n")
        .map(|calory_per_elve| {
            calory_per_elve
                .split("\n")
                .map(|calory| calory.parse().unwrap_or_default())
                .collect()
        })
        .collect();

    let sum_calory: Vec<i32> = calorylist_per_elve
        .clone()
        .iter_mut()
        .map(|calorylist| calorylist.iter().sum())
        .collect();

    println!("{:?}", sum_calory.iter().max());
    // part two
    let mut reverse = sum_calory.clone();
    reverse.sort();
    reverse.reverse();
    let biggest_three = reverse[0..3].to_vec();
    let sum_of_three: i32 = biggest_three.iter().sum();
    println!("{:?}", sum_of_three);
    Ok(())
}
