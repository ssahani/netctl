use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DhcpMode {
    #[default]
    No,
    Yes,
    Ipv4,
    Ipv6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IpNetwork {
    pub addr: IpAddr,
    pub prefix_len: u8,
}

impl IpNetwork {
    pub fn new(addr: IpAddr, prefix_len: u8) -> Result<Self> {
        Ok(Self { addr, prefix_len })
    }
}

impl fmt::Display for IpNetwork {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix_len)
    }
}

impl FromStr for IpNetwork {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (addr_str, prefix_str) = s.split_once('/').ok_or_else(|| Error::InvalidCidr {
            input: s.to_string(),
        })?;

        let addr = addr_str.parse().map_err(|_| Error::InvalidCidr {
            input: s.to_string(),
        })?;
        let prefix_len = prefix_str.parse().map_err(|_| Error::InvalidCidr {
            input: s.to_string(),
        })?;

        Self::new(addr, prefix_len)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MacAddress {
    octets: [u8; 6],
}

impl MacAddress {
    pub const fn new(octets: [u8; 6]) -> Self {
        Self { octets }
    }

    pub const fn octets(&self) -> [u8; 6] {
        self.octets
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
            self.octets[4],
            self.octets[5]
        )
    }
}

impl FromStr for MacAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 6 {
            return Err(Error::InvalidMacAddress {
                input: s.to_string(),
            });
        }

        let mut octets = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            octets[i] = u8::from_str_radix(part, 16).map_err(|_| Error::InvalidMacAddress {
                input: s.to_string(),
            })?;
        }

        Ok(Self::new(octets))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkState {
    Up,
    Down,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInfo {
    pub index: u32,
    pub name: String,
    pub state: LinkState,
    pub mtu: u32,
    pub mac_address: Option<MacAddress>,
    pub addresses: Vec<IpNetwork>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Route {
    pub destination: Option<IpNetwork>,
    pub gateway: Option<IpAddr>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_ipnetwork_parse_ipv4() {
        let net: IpNetwork = "192.168.1.10/24".parse().unwrap();
        assert_eq!(net.addr, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)));
        assert_eq!(net.prefix_len, 24);
    }

    #[test]
    fn test_ipnetwork_parse_ipv6() {
        let net: IpNetwork = "2001:db8::1/64".parse().unwrap();
        assert!(matches!(net.addr, IpAddr::V6(_)));
        assert_eq!(net.prefix_len, 64);
    }

    #[test]
    fn test_ipnetwork_parse_invalid_no_prefix() {
        let result: Result<IpNetwork> = "192.168.1.10".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_ipnetwork_parse_invalid_address() {
        let result: Result<IpNetwork> = "not.an.ip/24".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_ipnetwork_display() {
        let net = IpNetwork {
            addr: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            prefix_len: 8,
        };
        assert_eq!(net.to_string(), "10.0.0.1/8");
    }

    #[test]
    fn test_ipnetwork_roundtrip() {
        let original = "192.168.100.50/16";
        let net: IpNetwork = original.parse().unwrap();
        assert_eq!(net.to_string(), original);
    }

    #[test]
    fn test_macaddress_parse_valid() {
        let mac: MacAddress = "aa:bb:cc:dd:ee:ff".parse().unwrap();
        assert_eq!(mac.octets(), [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    }

    #[test]
    fn test_macaddress_parse_lowercase() {
        let mac: MacAddress = "00:11:22:33:44:55".parse().unwrap();
        assert_eq!(mac.octets(), [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_macaddress_parse_invalid_format() {
        let result: Result<MacAddress> = "aa:bb:cc:dd:ee".parse(); // Only 5 octets
        assert!(result.is_err());
    }

    #[test]
    fn test_macaddress_parse_invalid_hex() {
        let result: Result<MacAddress> = "zz:bb:cc:dd:ee:ff".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_macaddress_display() {
        let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert_eq!(mac.to_string(), "00:11:22:33:44:55");
    }

    #[test]
    fn test_macaddress_roundtrip() {
        let original = "aa:bb:cc:dd:ee:ff";
        let mac: MacAddress = original.parse().unwrap();
        assert_eq!(mac.to_string(), original);
    }

    #[test]
    fn test_dhcp_mode_default() {
        assert_eq!(DhcpMode::default(), DhcpMode::No);
    }

    #[test]
    fn test_route_creation() {
        let route = Route {
            destination: Some(
                IpNetwork::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)), 24).unwrap(),
            ),
            gateway: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
        };
        assert!(route.destination.is_some());
        assert!(route.gateway.is_some());
    }

    #[test]
    fn test_route_default_gateway() {
        let route = Route {
            destination: None, // Default route
            gateway: Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))),
        };
        assert!(route.destination.is_none());
        assert!(route.gateway.is_some());
    }
}
