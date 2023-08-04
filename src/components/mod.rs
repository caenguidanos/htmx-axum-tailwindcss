pub const HEADERS: &str = include_str!("./headers.html");
pub const HEADERS_SLOT: &str = "<output id=\"headers\"></output>";

pub const NAVBAR: &str = include_str!("./navbar.html");
pub const NAVBAR_SLOT: &str = "<output id=\"navbar\"></output>";

pub const HELLO: &str = include_str!("./hello.html");

pub const TIMESTAMP: &str = include_str!("./timestamp.html");

pub const SYSINFO: &str = include_str!("./sysinfo.html");

pub fn compose_base_route_with(src: &str) -> String {
    src.replace(HEADERS_SLOT, HEADERS)
        .replace(NAVBAR_SLOT, NAVBAR)
}
