use crate::Result;
use rsa::rand_core::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct KeyPair {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

impl KeyPair {
    pub fn generate(bits: usize) -> Result<Self> {
        let private_key = RsaPrivateKey::new(&mut OsRng, bits)?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            private_key,
            public_key,
        })
    }
}
