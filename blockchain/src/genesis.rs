use std::collections::LinkedList;
use wallet_crypto::{address, cbor, hash::{Blake2b256}};
use wallet_crypto::cbor::{ExtendedResult};
use wallet_crypto::config::{ProtocolMagic};
use std::{fmt};

use raw_cbor::{self, de::RawCbor};
use types;
use types::{HeaderHash, ChainDifficulty};

#[derive(Debug, Clone)]
pub struct BodyProof(Blake2b256);

impl cbor::CborValue for BodyProof {
    fn encode(&self) -> cbor::Value {
        cbor::CborValue::encode(&self.0)
    }
    fn decode(value: cbor::Value) -> cbor::Result<Self> {
        value.decode().and_then(|hash| Ok(BodyProof(hash))).embed("While decoding BodyProof")
    }
}
impl raw_cbor::de::Deserialize for BodyProof {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> raw_cbor::Result<Self> {
        raw_cbor::de::Deserialize::deserialize(raw).map(|h| BodyProof(h))
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub slot_leaders: LinkedList<address::StakeholderId>,
}
/*
impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
*/
impl cbor::CborValue for Body {
    fn encode(&self) -> cbor::Value {
        unimplemented!()
    }
    fn decode(value: cbor::Value) -> cbor::Result<Self> {
        let slot_leaders = cbor::CborValue::decode(value).embed("While decoding genesis::Body")?;
        Ok(Body { slot_leaders })
    }
}
impl raw_cbor::de::Deserialize for Body {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> raw_cbor::Result<Self> {
        let len = raw.array()?;
        assert_eq!(len, raw_cbor::Len::Indefinite);
        let mut slot_leaders = LinkedList::new();
        while {
            let t = raw.cbor_type()?;
            if t == raw_cbor::Type::Special {
                let special = raw.special()?;
                assert_eq!(special, raw_cbor::de::Special::Break);
                false
            } else {
                slot_leaders.push_back(raw_cbor::de::Deserialize::deserialize(raw)?);
                true
            }
        } {}
        Ok(Body { slot_leaders })
    }
}

#[derive(Debug, Clone)]
pub struct BlockHeader {
    pub protocol_magic: ProtocolMagic,
    pub previous_header: HeaderHash,
    pub body_proof: BodyProof,
    pub consensus: Consensus,
    pub extra_data: types::BlockHeaderAttributes,
}
impl fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!( f
            , "Magic: 0x{:?} Previous Header: {}"
            , self.protocol_magic
            , self.previous_header
            )
    }
}
impl BlockHeader {
    pub fn new(pm: ProtocolMagic, pb: HeaderHash, bp: BodyProof, c: Consensus, ed: types::BlockHeaderAttributes) -> Self {
        BlockHeader {
            protocol_magic: pm,
            previous_header: pb,
            body_proof: bp,
            consensus: c,
            extra_data: ed
        }
    }
}
impl cbor::CborValue for BlockHeader {
    fn encode(&self) -> cbor::Value {
        cbor::Value::Array(vec![
            cbor::CborValue::encode(&self.protocol_magic),
            cbor::CborValue::encode(&self.previous_header),
            cbor::CborValue::encode(&self.body_proof),
            cbor::CborValue::encode(&self.consensus),
            cbor::CborValue::encode(&self.extra_data),
        ])
    }
    fn decode(value: cbor::Value) -> cbor::Result<Self> {
        value.array().and_then(|array| {
            let (array, p_magic)    = cbor::array_decode_elem(array, 0).embed("protocol magic")?;
            let (array, prv_header) = cbor::array_decode_elem(array, 0).embed("Previous Header Hash")?;
            let (array, body_proof) = cbor::array_decode_elem(array, 0).embed("body proof")?;
            let (array, consensus)  = cbor::array_decode_elem(array, 0).embed("consensus")?;
            let (array, extra_data) = cbor::array_decode_elem(array, 0).embed("extra_data")?;
            if ! array.is_empty() { return cbor::Result::array(array, cbor::Error::UnparsedValues); }
            Ok(BlockHeader::new(p_magic, prv_header, body_proof, consensus, extra_data))
        }).embed("While decoding a genesis::BlockHeader")
    }
}
impl raw_cbor::de::Deserialize for BlockHeader {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> raw_cbor::Result<Self> {
        let len = raw.array()?;
        if len != raw_cbor::Len::Len(4) {
            return Err(raw_cbor::Error::CustomError(format!("Invalid BodyProof: recieved array of {:?} elements", len)));
        }
        let p_magic    = raw_cbor::de::Deserialize::deserialize(raw)?;
        let prv_header = raw_cbor::de::Deserialize::deserialize(raw)?;
        let body_proof = raw_cbor::de::Deserialize::deserialize(raw)?;
        let consensus  = raw_cbor::de::Deserialize::deserialize(raw)?;
        let extra_data = raw_cbor::de::Deserialize::deserialize(raw)?;

        Ok(BlockHeader::new(p_magic, prv_header, body_proof, consensus, extra_data))
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub body: Body,
    pub extra: cbor::Value
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        write!(f, "{:?}", self.body)
    }
}
impl cbor::CborValue for Block {
    fn encode(&self) -> cbor::Value {
        unimplemented!()
    }
    fn decode(value: cbor::Value) -> cbor::Result<Self> {
        value.array().and_then(|array| {
            let (array, header) = cbor::array_decode_elem(array, 0).embed("header")?;
            let (array, body)   = cbor::array_decode_elem(array, 0).embed("body")?;
            let (array, extra)  = cbor::array_decode_elem(array, 0).embed("extra")?;
            if ! array.is_empty() { return cbor::Result::array(array, cbor::Error::UnparsedValues); }
            Ok(Block { header: header, body: body, extra: extra })
        }).embed("While decoding genesis::Block")
    }
}
impl raw_cbor::de::Deserialize for Block {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> raw_cbor::Result<Self> {
        let len = raw.array()?;
        if len != raw_cbor::Len::Len(3) {
            return Err(raw_cbor::Error::CustomError(format!("Invalid Block: recieved array of {:?} elements", len)));
        }
        let header = raw_cbor::de::Deserialize::deserialize(raw)?;
        let body  = raw_cbor::de::Deserialize::deserialize(raw)?;
        let extra = {
            let _ = raw.array()?;
            let _ = raw.map()?;
            cbor::Value::Null
        };
        Ok(Block { header, body, extra })
    }
}

#[derive(Debug, Clone)]
pub struct Consensus {
    pub epoch: types::EpochId,
    pub chain_difficulty: ChainDifficulty,
}
impl cbor::CborValue for Consensus {
    fn encode(&self) -> cbor::Value {
        cbor::Value::Array(vec![
            cbor::CborValue::encode(&self.epoch),
            cbor::CborValue::encode(&self.chain_difficulty),
        ])
    }
    fn decode(value: cbor::Value) -> cbor::Result<Self> {
        value.array().and_then(|array| {
            let (array, epoch) = cbor::array_decode_elem(array, 0).embed("epoch")?;
            let (array, chain_difficulty) = cbor::array_decode_elem(array, 0).embed("chain_difficulty")?;
            if ! array.is_empty() { return cbor::Result::array(array, cbor::Error::UnparsedValues); }
            Ok(Consensus { epoch: epoch, chain_difficulty: chain_difficulty })
        }).embed("While decoding genesis::Consensus")
    }
}
impl raw_cbor::de::Deserialize for Consensus {
    fn deserialize<'a>(raw: &mut RawCbor<'a>) -> raw_cbor::Result<Self> {
        let len = raw.array()?;
        if len != raw_cbor::Len::Len(2) {
            return Err(raw_cbor::Error::CustomError(format!("Invalid Consensus: recieved array of {:?} elements", len)));
        }
        let epoch = *raw.unsigned_integer()? as u32;
        let chain_difficulty = raw_cbor::de::Deserialize::deserialize(raw)?;
        Ok(Consensus { epoch, chain_difficulty })
    }
}
