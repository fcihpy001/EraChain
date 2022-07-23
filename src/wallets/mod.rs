
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs;
use ring::signature::{ECDSA_P256_SHA256_FIXED_SIGNING, EcdsaKeyPair, KeyPair};
use tracing::info;
use crate::error::BlockchainError;
use crate::utils::{base58_encode, fc_deserialize, fc_serialize, new_private_key, ripemd160_digest, sha256_digest};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    private_key: Vec<u8>,
    public_key: Vec<u8>
}

impl Address {
    pub fn new() -> Self {
        let private_key = new_private_key();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING,private_key.as_ref()).unwrap();
        let public_key = key_pair.public_key().as_ref().to_vec();
        Self {
            private_key,
            public_key
        }

    }

    pub fn get_address_id(&self) -> String {
        let pub_key_hash = hash_pub_key(self.public_key.as_slice());
        let mut payload = vec![];
        payload.push(VERSION);
        payload.extend(pub_key_hash.as_slice());
        let checksum = checksum(payload.as_slice());
        payload.extend(checksum.as_slice());
        base58_encode(payload.as_slice())
    }
    pub fn get_private_key(&self) -> &[u8] {
        self.private_key.as_slice()
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }
}

pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = sha256_digest(pub_key);
    ripemd160_digest(&pub_key_sha256)
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = sha256_digest(payload);
    let second_sha = sha256_digest(&first_sha);
    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}
pub const WALLET_FILE: &str = "wallet.dat";

#[derive(Deserialize, Serialize)]
struct Wallet {
    address_list: HashMap<String, Address>
}

impl Wallet {
    pub fn new() -> Result<Self, BlockchainError> {
        Self.load_address_from_file()
    }
    pub fn create_addresds(&mut self) -> String {
        let address = Address::new();
        let address_id = address.get_address_id();
        self.address_list.insert(address_id.clone(), address);
        self.save_wallet_to_file().unwrap();
        address_id
    }

    pub fn get_address(&self, address_id: &str) -> Option<&Address> {
        self.address_list.get(address_id)
    }
    pub fn get_address_ids(&self) -> Vec<String> {
        self.address_list.keys().collect()
    }

    pub fn save_address_to_file(&self) -> Result<(), BlockchainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let address_serialize = fc_serialize(&self)?;
        fs::write(path, &address_serialize).unwrap();
        Ok(())
    }

    pub fn load_address_from_file() -> Result<Self, BlockchainError> {
        let path = current_dir().unwrap().join(WALLET_FILE);
        info!("Wallet path: {:?}", path);
        if !path.exists() {
            let list = Wallet {
                address_list: HashMap::new()
            };
            return Ok(list);
        }
        let address_serialize = fs::read(&path).unwrap();
        let list = fc_deserialize(&address_serialize);
        list
    }
}