use reqwest::Client;
use std::fs::File;
use std::io::{self, Write};
use tokio::time::{Duration, Instant};
use futures_util::StreamExt;


fn main(){
    use std::io::{stdin,stdout};
    let mut s=String::new();
    print!("Please enter url: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    let url:&str = &s.clone();
    let file = url.split('/').last().unwrap_or("filename_err"); //получаем имя файла нарезая ссылку
    let client = Client::new();

    let _ = download(url, file, client);

}

#[tokio::main]
pub async fn download(url: &str, file: &str, client: Client) -> Result<(), String> {
    let response = client.get(url).send().await.or(Err(format!("Connection failed")))?; //подключение к файлу из интернета по http
    let file_size = response.content_length().ok_or(format!("Getting file size failed"))?;

    let mut image = File::create(file).or(Err(format!("Making file failed")))?;
    let mut current_size: u64 = 0;
    let mut byte_stream = response.bytes_stream();

    let mut timer = Instant::now();

    while let Some(item) = byte_stream.next().await {
        let byte_part = item.or(Err(format!("Downloading file failed")))?;
        image.write_all(&byte_part).or(Err(format!("Filing failed")))?;
        let new_current_size = current_size + (byte_part.len() as u64);
        current_size = new_current_size;

        if timer.elapsed() >= Duration::from_secs(1) {
            println!("Bytes {}|{}...", current_size, file_size);
            timer = Instant::now();
        }
        io::stdout().flush().unwrap(); 
    }

    println!("Downloading is complete");

    return Ok(());

}