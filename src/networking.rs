use reqwest::blocking::Response;
use reqwest::StatusCode;

fn send_data_via_http (formatted_message: String, url: String){
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .body(formatted_message)
        .send();

    match res {
        Ok(done) => {
            //println!("WE DONE");
        }
        Err(err) => {
            //println!("ERROR: {}", err);
        }
    }
}