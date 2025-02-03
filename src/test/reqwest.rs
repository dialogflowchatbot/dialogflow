use core::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn download() -> Result<()> {
    let u = "https://huggingface.co/GanymedeNil/text2vec-large-chinese/resolve/main/tokenizer.json?download=true";
    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(5000))
        .read_timeout(Duration::from_millis(10000))
        .proxy(reqwest::Proxy::https("http://127.0.0.1:7897")?)
        .build()?;
    let res = client.get(u).send().await?;
    let total_size = res.content_length().unwrap();
    println!("Total size {total_size}");
    // let b = res.bytes().await?;
    // fs::write("./temp.file", b.as_ref()).await?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut file = File::create("./temp.file").await?;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        println!("Downloaded {new}");
        downloaded = new;
    }
    Ok(())
}
