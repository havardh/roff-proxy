use std::net::SocketAddr;

pub fn lookup(url: &str, port: u16) -> SocketAddr {
    use dns_lookup::lookup_host;

    match lookup_host(url) {
        Ok(mut hosts) => {
            match hosts.next().unwrap() {
                Ok(socket) => SocketAddr::new(socket, port),
                Err(e) => panic!("error: {:?}", e),
            }
        }
        Err(e) => panic!("error: {:?}", e),
    }
}
