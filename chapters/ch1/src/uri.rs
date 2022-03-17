pub struct URI {
    pub protocol: String,
    pub domain: String,
    pub port: u64, // TODO: figure out exact size
    pub path: String,
    pub query_param: String,
}

pub fn parse(uri: &str) -> Option<URI> {

    if uri.len() == 0 {
        return None;
    }

    // TODO: assume protocol is https
    let pro = uri[0..5];
    let rest = uri[8..];
    let rest_xs = rest.split("/");
    let d = rest_xs[0];
    let path = rest_xs.len() > 1 ? rest_xs[1] : "index.html";

    let uri = URI {
        protocol: String::from("https"),
        domain: String::from("example.com"),
        port: 80,
        path: String::from("/"),
        query_param: String::from("/")
    };
    return Some(uri);
}
