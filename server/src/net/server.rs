pub struct Server {
    ip_address: String,
    port: u16,
    password: String,
}

impl Server {
    pub fn from(args: crate::Args) -> Self {
        Server {
            ip_address: args.ip_address,
            port: args.port,
            password: args.password.unwrap(),
        }
    }
}
