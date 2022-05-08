use reqwest::header::CONTENT_TYPE;

pub fn send_data_via_http(formatted_message: String, url: &String) {
    let client = reqwest::blocking::Client::new();
    let res = client.post(url).header(CONTENT_TYPE, "application/json; charset=utf-8").body(formatted_message).send();

    match res {
        Ok(done) => {
            println!(
                "Sending confirmed. Received result: \n{}",
                done.text().unwrap()
            );
        }
        Err(err) => {
            println!("Error occurred: \n{}", err);
        }
    }
}
