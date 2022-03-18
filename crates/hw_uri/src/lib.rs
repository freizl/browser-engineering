use std::fmt;

pub struct URI {
    pub protocol: String,
    pub domain: String,
    pub port: u64, // TODO: figure out exact size
    pub path: String,
    pub query_param: String,
}

impl URI {
    pub fn use_tls(&self) -> bool {
        self.protocol == "https"
    }
    pub fn get_domain_port(&self) -> String {
        format!("{}:{}", self.domain, self.port)
    }
}

impl fmt::Display for URI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}://{}:{}{}",
            self.protocol, self.domain, self.port, self.path
        )
    }
}

pub fn parse(uri_input: &str) -> Option<URI> {
    if uri_input.len() == 0 {
        return None;
    }

    let xs: Vec<&str> = uri_input.split("://").collect();

    let protocol = xs[0];
    let rest_xs: Vec<&str> = xs[1].split("/").collect();
    let domain_str = rest_xs[0];
    let domain_str_xs: Vec<&str> = domain_str.split(":").collect();
    let mut port: u64 = if protocol == "https" { 443 } else { 80 };
    if domain_str_xs.len() == 2 {
        // TODO: unwrap is probably not ideal
        // shall error out if passing wrong format port number
        port = domain_str_xs[1].parse::<u64>().unwrap();
    }
    let domain = domain_str_xs[0];
    let mut path = "index.html";
    if rest_xs.len() > 1 {
        // TODO: query param and hash is not support yet.
        if rest_xs[1] != "/" {
            path = rest_xs[1];
        }
    }
    let mut path2 = String::from("/");
    path2.push_str(path);

    let uri = URI {
        protocol: protocol.to_string(),
        domain: domain.to_string(),
        // TODO: support send over 443 port
        port,
        path: path2,
        query_param: String::from(" "),
    };

    return Some(uri);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
