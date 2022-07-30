use std::io::Cursor;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn fetch_url(url: String, file_name: String) -> Result<()> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.134 Safari/537.36 Edg/103.0.1264.71")
        .send()
        .await
        .unwrap();
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(res.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}