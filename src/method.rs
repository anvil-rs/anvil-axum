use std::{fmt, str::FromStr};

use clap::ValueEnum;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Connect,
    Trace,
}

// print as lower case for mapping
impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Get => write!(f, "get"),
            Method::Post => write!(f, "post"),
            Method::Put => write!(f, "put"),
            Method::Delete => write!(f, "delete"),
            Method::Patch => write!(f, "patch"),
            Method::Options => write!(f, "options"),
            Method::Head => write!(f, "head"),
            Method::Connect => write!(f, "connect"),
            Method::Trace => write!(f, "trace"),
        }
    }
}

impl FromStr for Method {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Method::Get),
            "post" => Ok(Method::Post),
            "put" => Ok(Method::Put),
            "delete" => Ok(Method::Delete),
            "patch" => Ok(Method::Patch),
            "options" => Ok(Method::Options),
            "head" => Ok(Method::Head),
            "connect" => Ok(Method::Connect),
            "trace" => Ok(Method::Trace),
            _ => Err(std::fmt::Error),
        }
    }
}
