use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let session_cookie = "session=53616c7465645f5f7a93bee72655ac5d5dd0251b80c13d0b1e70c999a360ce05cf6fe049f722f71ff5c7bf86df9ab556e960da9ad0b9e77c0cc9cd1856c144af";
    let client = reqwest::Client::builder().build()?;
    let res = client
        .get("https://adventofcode.com/2022/day/1/input")
        .header("Cookie", session_cookie)
        .send()
        .await?
        .text()
        .await?;

    let lines = res.split("\n\n");
    let mut parsed: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
        .collect();

    parsed.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {:?}", parsed[0]);

    // part two
    println!("Part 2: {:?}", parsed.iter().take(3).sum::<u32>());
    Ok(())
}
