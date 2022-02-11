use dgc::DgcContainer;
use std::collections::HashMap;
use std::{error::Error, fs::read_to_string};

pub struct CertificateStore {
    certificates: HashMap<(String, String, String), DgcContainer>,
    trust_list: dgc::TrustList,
}

impl CertificateStore {
    pub fn new() -> Self {
        let certificates = HashMap::new();
        // We create a new Trustlist (container of "trusted" public keys)
        let trust_list = dgc::TrustList::default();
        Self {
            certificates,
            trust_list,
        }
    }

    pub fn load_trust_list(&mut self) -> Result<(), Box<dyn Error>> {
        let pub_keys = crate::pub_keys::read_file("trust_list.txt");

        for key in pub_keys {
            // We add the public key in the certificate to the trustlist
            self.trust_list
                .add_key_from_certificate(&key)
                .expect("Failed to add key from certificate");
        }
        Ok(())
    }
}
