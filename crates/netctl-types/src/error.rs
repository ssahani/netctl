use miette::Diagnostic;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("interface '{name}' not found")]
    #[diagnostic(code(netctl::not_found), help("Use 'netctl show' to list interfaces"))]
    InterfaceNotFound { name: String },

    #[error("invalid CIDR: {input}")]
    InvalidCidr { input: String },

    #[error("invalid MAC: {input}")]
    InvalidMacAddress { input: String },

    #[error("netlink error: {0}")]
    Netlink(String),

    #[error("D-Bus error: {0}")]
    Dbus(String),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Generic(String),
}

impl Error {
    pub fn netlink(msg: impl Into<String>) -> Self {
        Self::Netlink(msg.into())
    }

    pub fn dbus(msg: impl Into<String>) -> Self {
        Self::Dbus(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_interface_not_found() {
        let err = Error::InterfaceNotFound {
            name: "eth0".to_string(),
        };
        assert!(err.to_string().contains("eth0"));
    }

    #[test]
    fn test_error_invalid_cidr() {
        let err = Error::InvalidCidr {
            input: "192.168.1.1".to_string(),
        };
        assert!(err.to_string().contains("192.168.1.1"));
    }

    #[test]
    fn test_error_invalid_mac() {
        let err = Error::InvalidMacAddress {
            input: "zz:bb:cc".to_string(),
        };
        assert!(err.to_string().contains("zz:bb:cc"));
    }

    #[test]
    fn test_error_netlink() {
        let err = Error::netlink("connection failed");
        assert!(err.to_string().contains("connection failed"));
    }

    #[test]
    fn test_error_dbus() {
        let err = Error::dbus("method call failed");
        assert!(err.to_string().contains("method call failed"));
    }

    #[test]
    fn test_error_io_from() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err = Error::from(io_err);
        assert!(matches!(err, Error::Io(_)));
    }
}
