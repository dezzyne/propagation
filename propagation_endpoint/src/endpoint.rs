use std::fmt;

pub struct Endpoint<'a> {
    name: &'a str,
}

impl fmt::Display for Endpoint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Endpoint<'_> {
    fn new(name: &str) -> Endpoint {
        Endpoint { name }
    }
}
