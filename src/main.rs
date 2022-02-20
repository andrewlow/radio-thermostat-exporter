use error_chain::error_chain;
use std::env;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn read_thermostat() -> Result<String> {
    // grab the environment variable
    let tstat = env::var("TSTAT").unwrap();

    // create the URL
    let url = format!("http://{}/tstat", tstat);

    // perform the fetch
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // parse the json
    let payload = json::parse(&body).unwrap();

    // return the result as a string
    Ok(format!(
        "radio_thermostat_temperature {}\nradio_thermostat_state {}\n",
        payload["temp"], payload["tstate"]
    ))
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9864").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = read_thermostat().unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
