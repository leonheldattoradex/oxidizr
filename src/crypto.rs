use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use ring::digest::{digest, SHA256};
use ring::signature::UnparsedPublicKey;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KeyType {
    Ed25519,
    Rsa2048,
    Rsa3072,
    Rsa4096,
    Unknown,
}

impl FromStr for KeyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ed25519" => Ok(KeyType::Ed25519),
            "rsa2048" => Ok(KeyType::Rsa2048),
            "rsa3072" => Ok(KeyType::Rsa3072),
            "rsa4096" => Ok(KeyType::Rsa4096),
            _ => Ok(KeyType::Unknown),
        }
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key_type_str = match *self {
            KeyType::Ed25519 => "Ed25519",
            KeyType::Rsa2048 => "Rsa2048",
            KeyType::Rsa3072 => "Rsa3072",
            KeyType::Rsa4096 => "Rsa4096",
            KeyType::Unknown => "Unknown",
        };
        write!(f, "{}", key_type_str)
    }
}

pub struct Crypto;

impl Crypto {
    pub fn identify_rsa_key_type(public_key: &str) -> Result<KeyType, Box<dyn Error>> {
        let rsa = Rsa::public_key_from_pem(public_key.as_bytes())?;
        let key_length = rsa.size() * 8;
        match key_length {
            2048 => Ok(KeyType::Rsa2048),
            3072 => Ok(KeyType::Rsa3072),
            4096 => Ok(KeyType::Rsa4096),
            _ => Ok(KeyType::Unknown),
        }
    }

    pub fn sha256digest(data: &str) -> Vec<u8> {
        digest(&SHA256, data.as_bytes()).as_ref().to_vec()
    }

    pub fn sha256digest_hex(data: &str) -> String {
        hex::encode(Self::sha256digest(data))
    }

    pub fn rsa_pss_verify(public_key: &str, signature: &str, message: &str) -> bool {
        let rsa = Rsa::public_key_from_pem(public_key.as_bytes()).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let mut verifier =
            openssl::sign::Verifier::new(openssl::hash::MessageDigest::sha256(), &pkey).unwrap();
        verifier
            .set_rsa_padding(openssl::rsa::Padding::PKCS1_PSS)
            .unwrap();
        verifier.update(message.as_bytes()).unwrap();
        verifier.verify(signature.as_bytes()).is_ok()
    }

    pub fn ed25519_verify(public_key: &str, signature: &str, message: &str) -> bool {
        let public_key_bytes = hex::decode(public_key).unwrap();
        let unparsed_key = UnparsedPublicKey::new(&ring::signature::ED25519, public_key_bytes);
        unparsed_key
            .verify(message.as_bytes(), signature.as_bytes())
            .is_ok()
    }
}
