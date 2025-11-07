pub mod tcp {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    use crate::errors::errors::{self, G_ERROR};

    pub fn go_to(host: &str, port: u16, path: &str) -> Result<String, errors::Handler> {
        let split_host: Vec<&str> = host.split("//").collect();

        if !split_host.get(0).unwrap().contains(&"gopher:") {
            return Err(errors::Handler::throw(G_ERROR::NOT_GOPHER));
        }

        let url = format!("{}:{}", split_host.get(1).unwrap(), port);
        let stream = TcpStream::connect(&url);

        match stream {
            Ok(mut res) => {
                let selector = format!("{}\r\n", path);
                res.write(selector.as_bytes()).unwrap();
                res.flush().unwrap();

                let mut data = Vec::new();

                match res.read_to_end(&mut data) {
                    Ok(_) => Ok(String::from_utf8_lossy(&data).to_string()),
                    Err(_) => Err(errors::Handler::throw(G_ERROR::OTHER)
                        .custom_error("Failed to read response".to_string())),
                }
            }

            Err(e) => Err(errors::Handler::throw(G_ERROR::OTHER)
                .custom_error("Not a gopher hole".to_string())),
        }
    }
}
