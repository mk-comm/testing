pub struct Proxy {
    pub ip: String,
    pub username: String,
    pub password: String,
}

impl Proxy {
    pub fn new(ip: String, username: String, password: String) -> Self {
        Proxy {
            ip,
            username,
            password,
        }
    }
}