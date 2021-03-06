//! The Host request header, defined in RFC 2616, Section 14.23.

use std::io::Reader;
use std::fmt;

/// A simple little thing for the host of a request
#[derive(Clone, PartialEq, Eq)]
pub struct Host {

    /// The name of the host that was requested
    pub name: String,

    /// If unspecified, assume the default port was used (80 for HTTP, 443 for HTTPS).
    /// In that case, you shouldn't need to worry about it in URLs that you build, provided you
    /// include the scheme.
    pub port: Option<u16>,
}

impl fmt::Show for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.port {
            Some(port) => write!(f, "{}:{}", self.name, port),
            None => f.write_str(self.name[]),
        }
    }
}

impl super::HeaderConvertible for Host {
    fn from_stream<R: Reader>(reader: &mut super::HeaderValueByteIterator<R>) -> Option<Host> {
        let s = reader.collect_to_string();
        // TODO: this doesn't support IPv6 address access (e.g. "[::1]")
        // Do this properly with correct authority parsing.
        let mut hi = s[].splitn(1, ':');
        Some(Host {
            name: String::from_str(hi.next().unwrap()),
            port: match hi.next() {
                Some(name) => name.parse(),
                None => None,
            },
        })
    }

    fn http_value(&self) -> String {
        format!("{}", self)
    }
}
