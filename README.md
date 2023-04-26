# proxy-parser

## Technical Case

There is a file with an undefined name. Inside there are (or not) proxy server data in the format
```plain
# common format: protocol://username:password@ip:port
# and without auth: protocol://ip:port
http://127.0.0.12:80
https://ernest:halimov@192.168.0.15:443
socks5://true:gigachad@127.0.11.4:1080
```

You need to parse the file. All data should be stored in memory, in a form convenient for transmission by the program.

## Solution

```rust
use proxy_parser::Proxy;

fn main_example() {
    let file = std::fs::read("your-file-with-proxy").unwrap();
    let proxies: Vec<Proxy> = Proxy::from_str(String::from_utf8(file).unwrap()); // Vec<Proxy>
}
```

## Usage 

Install using cargo from this repository. 

### Simple

```rust
fn main() {
    let proxies = proxy_parser::Proxy::parse_str("https://username:password@127.0.0.1:125\nhttps://username:password@127.0.0.1:124");
    
    assert_eq!(proxies.len(), 2);
     
    assert_eq!(proxies[0].port(), 125);
    assert_eq!(proxies[0].scheme(), "https://127.0.0.1:125".to_string());
  
    assert_eq!(proxies[1].port(), 124);
    assert_eq!(proxies[1].scheme(), "https://127.0.0.1:124".to_string());
}
```

### P.S.

Currently, there are only 4 versions of the IP parsing available, and only the format that was presented above.
