use attohttpc::Method;
use std::collections::HashMap;

pub trait Endpoint {
    fn path(&self) -> String;
    fn method(&self) -> Method;
    fn params(&self) -> HashMap<&str, String>;
}
