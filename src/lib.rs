#![warn(missing_docs)]
//#![deny(warnings)]
//! Provides a client interface for [AWS X-Ray](https://aws.amazon.com/xray/)

use log::warn;
use serde::Serialize;
use std::{
    net::{SocketAddr, UdpSocket},
    result::Result as StdResult,
    str::FromStr,
    sync::Arc,
};

mod epoch;
mod error;
mod header;
mod hexbytes;
mod segment;
mod segment_id;
mod trace_id;

pub use crate::{
    epoch::Seconds, error::Error, header::Header, segment::*, segment_id::SegmentId,
    trace_id::TraceId,
};

/// Type alias for Results which may return `xray::Errors`
pub type Result<T> = StdResult<T, Error>;

/// X-Ray daemon client interface
#[derive(Debug)]
pub struct Client {
    socket: Arc<UdpSocket>,
}

impl FromStr for Client {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let addr: SocketAddr = s.parse()?;
        Self::new(addr)
    }
}

impl Client {
    const HEADER: &'static [u8] = br#"{"format": "json", "version": 1}\n"#;

    /// Return a new X-Ray client connected
    /// to the provided `addr`
    pub fn new(addr: SocketAddr) -> Result<Self> {
        let socket = Arc::new(UdpSocket::bind(&[([0, 0, 0, 0], 0).into()][..])?);
        socket.set_nonblocking(true)?;
        socket.connect(&addr)?;
        Ok(Client { socket })
    }

    #[inline]
    fn packet<S>(data: S) -> Result<Vec<u8>>
    where
        S: Serialize,
    {
        let bytes = serde_json::to_vec(&data)?;
        Ok([Self::HEADER, &bytes].concat())
    }

    /// send a segment to the xray daemon this client is connected to
    pub fn send<S>(&self, data: &S) -> Result<()>
    where
        S: Serialize,
    {
        self.socket.send(&Self::packet(data)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn client_prefixes_packets_with_header() {
        assert_eq!(
            Client::packet(serde_json::json!({
                "foo": "bar"
            }))
            .unwrap(),
            br#"{"format": "json", "version": 1}\n{"foo":"bar"}"#.to_vec()
        )
    }
}
