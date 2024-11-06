use hex;
use primitive_types::U256;
use std::str::FromStr;
use std::string::ToString;
#[derive(Debug, Clone)]
pub struct SU256 {
    pub v: U256,
}

#[derive(Debug, PartialEq, Eq)]

pub struct SU256ParseError;

impl FromStr for SU256 {
    type Err = SU256ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match U256::from_str_radix(s, 16) {
            Ok(n) => Ok(Self { v: n }),
            Err(_) => Err(SU256ParseError),
        }
    }
}

impl ToString for SU256 {
    fn to_string(&self) -> String {
        let bytes = self.v.to_big_endian();
        hex::encode(bytes)
    }
}
