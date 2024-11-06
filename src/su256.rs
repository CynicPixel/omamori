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

impl SU256 {
    pub fn add_mod(&self, b: &Self, p: &Self) -> Self {
        let x1 = self.v.checked_rem(p.v).expect("modulo");
        let x2 = b.v.checked_rem(p.v).expect("modulo");

        let (mut x3, over) = x1.overflowing_add(x2);

        if over {
            x3 = x3
                .checked_add(
                    U256::MAX
                        .checked_sub(p.v)
                        .expect("sub")
                        .checked_add(U256::from_big_endian(&[1]))
                        .expect("conversion"),
                )
                .expect("add");
        }
        x3 = x3.checked_rem(p.v).expect("modulo");
        Self { v: x3 }
    }
    pub fn sub_mod(&self, b: &self, p: &self) -> Self {}
    pub fn mul_mod(&self, b: &self, p: &self) -> Self {}
    pub fn div_mod(&self, b: &self, p: &self) -> Self {}
    pub fn exp_mod(&self, b: &self, p: &self) -> Self {}
}
