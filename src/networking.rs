
fn send_data_via_http (formatted_message: String, url: String)->Result<(),reqwest::Error>{
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .body(formatted_message)
        .send()?;
    Ok(())
}