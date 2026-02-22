use core::fmt;
use core::net::SocketAddr;

use ipnet::IpNet;

/// Peer connection filter for [`super::TcpServer`].
///
/// Controls which incoming connections are allowed through the proxy.
pub enum PeerFilter {
    /// Allow all incoming connections.
    All,
    /// Allow connections only from a specific IP subnet.
    Subnet(IpNet),
    /// Allow connections based on custom logic.
    ///
    /// The function receives the peer's socket address and returns
    /// `true` to allow the connection or `false` to reject it.
    Custom(Box<dyn Fn(SocketAddr) -> bool + Send + Sync>),
}

impl PeerFilter {
    /// Check whether a peer address is allowed by this filter.
    #[must_use]
    pub fn is_allowed(&self, addr: SocketAddr) -> bool {
        match self {
            Self::All => true,
            Self::Subnet(net) => net.contains(&addr.ip()),
            Self::Custom(f) => f(addr),
        }
    }
}

impl fmt::Debug for PeerFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => f.write_str("PeerFilter::All"),
            Self::Subnet(net) => f.debug_tuple("PeerFilter::Subnet").field(net).finish(),
            Self::Custom(_) => f.write_str("PeerFilter::Custom(..)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::*;

    #[test]
    fn test_peer_filter_all() {
        let filter = PeerFilter::All;
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 1234);

        assert!(filter.is_allowed(addr));
    }

    #[test]
    fn test_peer_filter_subnet_v4() {
        let filter = PeerFilter::Subnet("192.168.1.0/24".parse().unwrap());
        let allowed = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 50)), 1234);
        let rejected = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 1234);

        assert!(filter.is_allowed(allowed));
        assert!(!filter.is_allowed(rejected));
    }

    #[test]
    fn test_peer_filter_subnet_v6() {
        let filter = PeerFilter::Subnet("fe80::/10".parse().unwrap());
        let allowed = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)), 1234);
        let rejected =
            SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 1)), 1234);

        assert!(filter.is_allowed(allowed));
        assert!(!filter.is_allowed(rejected));
    }

    #[test]
    fn test_peer_filter_subnet_family_mismatch() {
        let filter = PeerFilter::Subnet("192.168.1.0/24".parse().unwrap());
        let v6_addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 1234);

        assert!(!filter.is_allowed(v6_addr));
    }

    #[test]
    fn test_peer_filter_custom() {
        let filter = PeerFilter::Custom(Box::new(|addr| addr.port() == 9999));
        let allowed = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 9999);
        let rejected = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)), 1234);

        assert!(filter.is_allowed(allowed));
        assert!(!filter.is_allowed(rejected));
    }
}
