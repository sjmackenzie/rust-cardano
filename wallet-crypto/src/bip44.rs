use hdpayload::{Path};

pub const BIP44_PURPOSE   : u32 = 0x8000002C;
pub const BIP44_COIN_TYPE : u32 = 0x80000717;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Addressing {
    pub account: u32,
    pub change: u32,
    pub index: u32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AddrType {
    Internal,
    External,
}

impl Addressing {
    pub fn new(account: u32, typ: AddrType) -> Self {
        let change = match typ {
                        AddrType::Internal => 1,
                        AddrType::External => 0,
                    };
        Addressing { account: 0x80000000 | account, change: change, index: 0 }
    }

    pub fn to_path(&self) -> Path {
        Path::new(vec![BIP44_PURPOSE, BIP44_COIN_TYPE, self.account, self.change, self.index])
    }

    pub fn address_type(&self) -> AddrType {
        if self.change == 0 {
            AddrType::External
        } else {
            AddrType::Internal
        }
    }

    pub fn from_path(path: Path) -> Option<Self> {
        if path.as_ref().len() != 5 { return None; }
        if path.as_ref()[0] != BIP44_PURPOSE   { return None; }
        if path.as_ref()[1] != BIP44_COIN_TYPE { return None; }
        if path.as_ref()[2]  < 0x80000000      { return None; }

        Some(Addressing {
            account: path.as_ref()[2],
            change:  path.as_ref()[3],
            index:   path.as_ref()[4],
        })
    }

    pub fn incr(&self, incr: u32) -> Option<Self> {
        if incr >= 0x80000000 { return None; }
        let mut addr = self.clone();
        addr.index += incr;
        Some(addr)
    }

    pub fn next_chunks(&self, chunk_size: usize) -> Vec<Self> {
        let mut v = Vec::with_capacity(chunk_size);
        for i in 0..chunk_size {
            match self.incr(i as u32) {
                None => break,
                Some(r) => v.push(r)
            }
        }
        v
    }
}