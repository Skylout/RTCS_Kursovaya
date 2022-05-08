use reqwest::header::CONTENT_TYPE;

pub fn send_data_via_http(formatted_message: String, url: &String, prefix: &String, endpoint: &String) {
    let client = reqwest::blocking::Client::new();

    let mut destination_url = url.clone();
    destination_url.push_str(prefix);
    destination_url.push_str(endpoint);

    let res = client.post(&destination_url).header(CONTENT_TYPE, "application/json; charset=utf-8").body(formatted_message.clone()).send();

    match res {
        Ok(done) => {
            println!(
                "Sending confirmed. \nSent data: \n{}\nReceived result: \n{}\n",
                &formatted_message,done.text().unwrap()
            );
        }
        Err(err) => {
            println!("Destination: {}\nError occurred: \n{}", &destination_url,err);
        }
    }
}
