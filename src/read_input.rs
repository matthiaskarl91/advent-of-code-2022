use reqwest::{Error, Response};

pub async fn read_input(url: &str) -> Result<String, Error> {
    let session_cookie = "session=53616c7465645f5f7a93bee72655ac5d5dd0251b80c13d0b1e70c999a360ce05cf6fe049f722f71ff5c7bf86df9ab556e960da9ad0b9e77c0cc9cd1856c144af";
    let client = reqwest::Client::builder().build()?;
    client
        .get(url)
        .header("Cookie", session_cookie)
        .send()
        .await?
        .text()
        .await
}

pub async fn read_stream(url: &str) -> Result<Response, Error> {
    let session_cookie = "session=53616c7465645f5f7a93bee72655ac5d5dd0251b80c13d0b1e70c999a360ce05cf6fe049f722f71ff5c7bf86df9ab556e960da9ad0b9e77c0cc9cd1856c144af";
    let client = reqwest::Client::builder().build()?;
    client
        .get(url)
        .header("Cookie", session_cookie)
        .send()
        .await
}
