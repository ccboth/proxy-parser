// use proxy_parser;
use super::*;
#[test]
fn parse_full_string() {
    // proxy_parser::
    let proxies = Proxy::parse_str(r#"
https://username:password@127.0.0.1:125
socks5://username:asdasdfasdf@127.0.0.1:748
http://127.0.0.1:125
socks5://127.0.0.1:748"#);

    assert_eq!(proxies.len(), 4);
    assert_eq!(proxies[0].port(), 125);
    assert_eq!(proxies[1].port(), 748);
    assert_eq!(proxies[2].port(), 125);
    assert_eq!(proxies[3].port(), 748);

    assert_eq!(proxies[0].password, Some("password".to_string()));
    assert_eq!(proxies[1].password, Some("asdasdfasdf".to_string()));
    assert_eq!(proxies[2].password, None);
    assert_eq!(proxies[3].password, None);

    let proxies = Proxy::parse_str("https://username:password@127.0.0.1:125");
    assert_eq!(proxies.len(), 1);
}

#[test]
fn from_str() {
    let proxy_without_auth = Proxy::from_str("socks5://127.0.0.1:80").unwrap();
    assert_eq!(proxy_without_auth.password, None);
    assert_eq!(proxy_without_auth.ip.to_string(), "127.0.0.1");
    assert_eq!(proxy_without_auth.port, 80);
    assert_eq!(proxy_without_auth.protocol, "socks5".to_string());
    assert_eq!(proxy_without_auth.password, None);
    assert_eq!(proxy_without_auth.username, None);

    let proxy_with_auth = Proxy::from_str("https://login:pass@192.168.0.1:1080").unwrap();
    // assert_eq!(proxy_with_auth.password, None);
    assert_eq!(proxy_with_auth.ip.to_string(), "192.168.0.1");
    assert_eq!(proxy_with_auth.port, 1080);
    assert_eq!(proxy_with_auth.protocol, "https");
    assert_eq!(proxy_with_auth.username.clone().unwrap(), "login");
    assert_eq!(proxy_with_auth.password.clone().unwrap(), "pass");
}