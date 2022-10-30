//! A Request represents a Gopher request made by a client. phd can
//! serve directory listings as Gopher Menus, plain text files as
//! Text, binary files as downloads, Gophermap files as menus, or
//! executable files as dynamic content.

use ipstuff::MaskedIpv4;

use crate::Result;
use std::{fs, net::Ipv4Addr};

/// This struct represents a single gopher request.
#[derive(Debug, Clone)]
pub struct Request {
    /// Gopher selector requested
    pub selector: String,
    /// Search query string, if any.
    pub query: String,
    /// Root directory of the server. Can't serve outside of this.
    pub root: String,
    /// Hosts (LAN,WAN,localhost)
    pub hosts: (String,String,String),
    /// Port of the currently running server.
    pub port: u16,
    /// remote ipv4, used for comparison since gopher voids connections
    pub rem: Option<Ipv4Addr>,
    /// Masked ip to use for comparing
    pub masked: MaskedIpv4,
    /// The gopher type of the request
    pub gph_char: char
}

#[cfg(target_os="linux")]
fn get_subnet() -> String {
    use std::process::Command;
    let cout = Command::new("sh").args(vec!["-c","ifconfig | grep -i mask | grep $(hostname -I | awk '{print $1}') | awk '{print $4}'"]).output();
    if let Ok(c) = cout {
        String::from_utf8_lossy(&c.stdout).into_owned().trim().into()
    } else {
        "255.255.255.255".into()
    }
}

#[cfg(not(target_os="linux"))]
fn get_subnet() -> String {
    "255.255.255.255".into()
}

impl Request {
    /// Try to get the host (either local ip, LAN ip, or WAN ip)
    pub fn get_host(&self) -> String {
        let rem = self.rem.unwrap_or_else( || "127.0.0.1".parse().unwrap());
        if self.masked.ip == rem {
            self.hosts.2.clone()
        } else if self.masked.contains(rem) {
            self.hosts.0.clone()
        } else {
            self.hosts.1.clone()
        }
    }

    /// Try to create a new request state object.
    pub fn from(host: &str,lcl: &str, port: u16, root: &str, remote: Option<Ipv4Addr>) -> Result<Request> {
        let mask = get_subnet();
        let fmt = format!("{} {}",host,mask).into_boxed_str();
        let masked = MaskedIpv4::from_network_str(&fmt).unwrap();
        Ok(Request {
            hosts: (host.into(),lcl.into(),"127.0.0.1".into()),
            port,
            root: fs::canonicalize(root)?.to_string_lossy().into(),
            selector: String::new(),
            query: String::new(),
            masked,
            rem: remote,
            gph_char: ' '
        })
    }

    /// Path to the target file on disk requested by this request.
    pub fn file_path(&self) -> String {
        format!(
            "{}/{}",
            self.root.to_string().trim_end_matches('/'),
            self.selector.replace("..", ".").trim_start_matches('/')
        )
    }

    /// Path to the target file relative to the server root.
    pub fn relative_file_path(&self) -> String {
        self.file_path().replace(&self.root, "")
    }

    /// Set selector + query based on what the client sent.
    pub fn parse_request(&mut self, line: &str) {
        println!("parsing request: {}",line);
        self.query.clear();
        self.selector.clear();
        if let Some((i, _)) = line
            .chars()
            .enumerate()
            .find(|&(_, c)| c == '\t' || c == '?')
        {
            if line.len() > i {
                self.query.push_str(&line[i + 1..]);
                self.selector.push_str(&line[..i]);
                return;
            }
        }
        self.selector.push_str(line);

        // strip trailing /
        if let Some(last) = self.selector.chars().last() {
            if last == '/' {
                self.selector.pop();
            }
        }
    }
}
