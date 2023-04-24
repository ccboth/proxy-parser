use std::str::FromStr;
use std::net::IpAddr;

pub struct Proxy {
    protocol: String,
    ip: IpAddr,
    port: u16,
    username: Option<String>,
    password: Option<String>
}

impl Proxy {
    
    /// This function returns a reference to the IP address of a struct.
    /// 
    /// Returns:
    /// 
    /// The `ip` field of the struct as a reference to an `IpAddr` object.
    pub fn ip(&self) -> &IpAddr {
        return &self.ip;
    }

    pub fn protocol(&self) -> &str {
        return self.protocol.as_str();
    }

    pub fn port(&self) -> u16{
        return self.port;
    }

    pub fn username(&self) -> &Option<String> {
        return &self.username;
    }

    pub fn password(&self) -> &Option<String> {
        return &self.password;
    }

    pub fn scheme(&self) -> String {
        return format!("{}://{}:{}", self.protocol, self.ip(), self.port);
    }

    /// This function parses a string into a vector of a certain type, ignoring lines starting with "#"
    /// and discarding lines that cannot be parsed.
    /// 
    /// Arguments:
    /// 
    /// * `string`: A generic type S that implements the ToString trait, which represents the input
    /// string that contains a list of proxies separated by newline characters.
    /// 
    /// Returns:
    /// 
    /// The function `parse_str` returns a `Vec<Self>`, which is a vector of the type that the function
    /// is defined in.
    /// 
    /// # Example 
    /// ```
    /// let proxies = proxy_parser::Proxy::parse_str(r#"https://username:password@127.0.0.1:125
    /// https://username:password@127.0.0.1:124"#);
    /// 
    /// assert_eq!(proxies.len(), 2);
    /// 
    /// assert_eq!(proxies[0].port(), 125);
    /// assert_eq!(proxies[0].scheme(), "https://127.0.0.1:125".to_string());
    /// 
    /// assert_eq!(proxies[1].port(), 124);
    /// assert_eq!(proxies[1].scheme(), "https://127.0.0.1:124".to_string());
    /// ```
    pub fn parse_str<S: ToString>(string: S) -> Vec<Self> {
        let raw_string = string.to_string();
        let proxy_splitted = raw_string.split("\n");
        let mut result_proxies_array: Vec<Self> = vec![];
        for proxy_line in proxy_splitted {
            if proxy_line.get(0..1) == Some("#") {continue;}
            match Self::from_str(proxy_line) {
                Ok(proxy) => result_proxies_array.push(proxy),
                Err(_) => {}
            };
        }
        return result_proxies_array;
    }

}

impl FromStr for Proxy {
    type Err = ();

    /// This function parses a string representing a proxy and returns a struct containing its protocol,
    /// IP address, port, username, and password.
    /// 
    /// Arguments:
    /// 
    /// * `proxy_line`: A string slice that represents a proxy configuration line in the format
    /// "protocol://username:password@host:port".
    /// 
    /// Returns:
    /// 
    /// a `Result<Self, Self::Err>` where `Self` refers to the type of the struct being implemented and
    /// `Self::Err` refers to the associated error type.
    fn from_str(proxy_line: &str) -> Result<Self, Self::Err> {
        let mut scheme_data = proxy_line.split("://");
        let protocol = match scheme_data.next() {
            Some(scheme) => scheme.to_string(),
            None => return Err(())
        };

        let proxy_data = match scheme_data.next() {
            Some(data) => data,
            None => return Err(())
        };
        let mut credits_host = proxy_data.split("@");
        let mut username: Option<String> = None;
        let mut password: Option<String> = None;

        if proxy_data.contains("@") {
            let credits = credits_host.next().unwrap();
            let mut user_pass = credits.split(":");
            let field_user = user_pass.next();
            let field_pass = user_pass.next();
            if field_user.is_none() {}
            else {
                username = Some(field_user.unwrap().to_string());
                password = match field_pass {
                    Some(str) => Some(str.to_string()),
                    None => None
                };
            }
        }
        let host = match credits_host.next() {
            Some(host) => host,
            None => return Err(())
        };
        let mut ip_port = host.split(":");
        let ip = match ip_port.next() {
            Some(ip) => match IpAddr::from_str(ip) {
                Ok(ipaddr) => ipaddr,
                Err(_) => return Err(())
            },
            None => return Err(())
        };
        let port = match ip_port.next() {
            Some(port) => match u16::from_str(port) {
                Ok(port) => port,
                Err(_) => return Err(())
            },
            None => return Err(())
        };
        return Ok(Self { protocol, ip, port, username, password});
    }
}

// impl ToString for Proxy {
//     fn to_string(&self) -> String {
//         if self.username.is_none() {
//             return self.scheme();
//         }
//         // else if self.password.is_none() { // Тут хз, на свой страх и риск.
//         //     return format!("{}://{}@{}:{}", self.scheme(), self.username.clone().unwrap(), self.ip(), self.port);
//         // }
//         else {
//             return format!("{}://{}:{}@{}:{}", self.scheme(), self.username().clone().unwrap(), self.password().clone().unwrap(), self.ip(), self.port());
//         }
//     }
// }

#[cfg(test)]
mod tests {
    mod parser;
    use super::*;
}
