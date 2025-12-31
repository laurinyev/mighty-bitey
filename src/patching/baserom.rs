
use std::io::Read;

use serde::*;
use sha3::*;

// sha3 hashes for all ROM types
static JP_HASH:        &str = "afb83e2cae77ab849d7349aa1b3be4e82de4523d3af24d9fd99ac005e56426e2";
static EN_1_0_0_HASH: &str = "UNKNOWN";
static EN_1_1_0_HASH: &str = "UNKNOWN";
static EN_1_2_0_HASH: &str = "UNKNOWN";
static EN_1_3_0_HASH: &str = "cc5b53f8e2bf354fff61f0dc7c28f0693a3d7c5ab01d443ec1774cb2f5bc20de";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum BaseromType {
    #[default]
    JP,
    En1_0_0,
    En1_1_0,
    En1_2_0,
    En1_3_0
}

impl BaseromType {
    pub fn get_expected_hash(&self) -> &str {
        match self {
            BaseromType::JP => JP_HASH,
            BaseromType::En1_0_0 => EN_1_0_0_HASH,
            BaseromType::En1_1_0 => EN_1_1_0_HASH,
            BaseromType::En1_2_0 => EN_1_2_0_HASH,
            BaseromType::En1_3_0 => EN_1_3_0_HASH,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Baserom {
    pub ver: BaseromType,
    pub hash: String,
    pub bytes: Vec<u8>
}

impl Baserom {
    pub fn load(path: &str, br_type: &BaseromType) -> Self{

        if let Ok(mut file) = std::fs::File::open(&path) {
            let mut buf = Vec::<u8>::new();
            file.read_to_end(&mut buf).expect("Couldn't read base ROM");

            let mut sha3 = Sha3_256::new();
            sha3.update(&buf);
            let hash = sha3.finalize();

            Baserom { 
                ver:  br_type.clone(),
                hash:  hex::encode(hash),
                bytes: buf
            }
        } else {
            Baserom { 
                ver:  br_type.clone(), 
                hash:  "".to_string(), 
                bytes: vec![] 
            }
        }
    }
}