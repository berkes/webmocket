use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub port: u16,
    pub address: IpAddr,
    pub ws_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 3000,
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            ws_path: String::from("/ws"),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let default = Config::default();
        Self {
            port: Self::env_or_default("WEBMOCKET_PORT", default.port),
            address: Self::env_or_default("WEBMOCKET_ADDR", default.address),
            ws_path: Self::env_or_default("WEBMOCKET_WS_PATH", default.ws_path.clone()),
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.address, self.port)
    }

    fn env_or_default<T: FromStr>(env_name: &str, default: T) -> T {
        if let Ok(val) = env::var(env_name) {
            val.parse::<T>().unwrap_or(default)
        } else {
            default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_has_defaults() {
        let sut = Config::default();
        assert_eq!("127.0.0.1".parse(), Ok(sut.address));
        assert_eq!(3000, sut.port);
        assert_eq!(String::from("/ws"), sut.ws_path)
    }

    #[test]
    fn socket_addr() {
        let sut = Config::default();
        assert_eq!("127.0.0.1:3000".parse(), Ok(sut.socket_addr()));
    }

    #[test]
    fn from_env_none_provided() {
        temp_env::with_vars_unset(
            vec!["WEBMOCKET_ADDR", "WEBMOCKET_PORT", "WEBMOCKET_WS_PATH"],
            || {
                let sut = Config::from_env();
                assert_eq!(Config::default(), sut);
            },
        );
    }

    #[test]
    fn from_env_all_provided() {
        temp_env::with_vars(
            vec![
                ("WEBMOCKET_ADDR", Some("127.0.0.2")),
                ("WEBMOCKET_PORT", Some("3001")),
                ("WEBMOCKET_WS_PATH", Some("/to/ws")),
            ],
            || {
                let sut = Config::from_env();

                assert_eq!("127.0.0.2".parse(), Ok(sut.address));
                assert_eq!(3001, sut.port);
                assert_eq!(String::from("/to/ws"), sut.ws_path)
            },
        )
    }

    #[test]
    fn from_env_some_provided() {
        temp_env::with_vars(
            vec![
                ("WEBMOCKET_ADDR", Some("127.0.0.2")),
                ("WEBMOCKET_PORT", None),
                ("WEBMOCKET_WS_PATH", None),
            ],
            || {
                let sut = Config::from_env();

                assert_eq!("127.0.0.2".parse(), Ok(sut.address));
                assert_eq!(3000, sut.port);
                assert_eq!(String::from("/ws"), sut.ws_path)
            },
        )
    }

    #[test]
    fn from_env_nonsense_provided() {
        temp_env::with_vars(
            vec![
                ("WEBMOCKET_ADDR", Some("nonsense")),
                ("WEBMOCKET_PORT", Some("fourtytwo")),
                ("WEBMOCKET_WS_PATH", None),
            ],
            || {
                let sut = Config::from_env();
                assert_eq!(Config::default(), sut);
            },
        )
    }
}
