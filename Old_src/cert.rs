use dgc::{DgcContainer, SignatureValidity};
use std::collections::HashMap;
use std::error::Error;

pub struct CertificateStore {
    // HashMap of the person with certificate data (firstname, full_name, date_of_birth)
    certificates: HashMap<(String, String, String), DgcContainer>,
    trust_list: dgc::TrustList,
}

impl CertificateStore {
    pub fn new() -> Self {
        let certificates = HashMap::new();
        let trust_list = dgc::TrustList::default();
        Self {
            certificates,
            trust_list,
        }
    }

    pub fn load_trust_list(&mut self) -> Result<(), Box<dyn Error>> {
        let pub_keys = crate::read_ops::read_file("trust_list.txt");

        for key in pub_keys {
            // We add the public key in the certificate to the trustlist
            self.trust_list
                .add_key_from_certificate(&key)
                .expect("Failed to add key from certificate");
        }
        Ok(())
    }

    pub fn add_certificate(
        &mut self,
        raw_cert_data: &str,
    ) -> Result<(String, String, SignatureValidity), Box<dyn Error>> {
        // Now we can validate the signature (this returns)
        let (mut certificate_container, signature_validity) =
            dgc::validate(raw_cert_data, &self.trust_list).expect("Cannot parse certificate data");

        // Calls `expand_values()` to resolve all the IDs against a well known valueset embedded in the library
        certificate_container.expand_values();

        let dgc_name = certificate_container.certs.get(&1).unwrap();
        let firstname = dgc_name.name.forename.clone().unwrap_or("".into());
        let surname = dgc_name.name.surname.clone().unwrap_or("".into());
        let date_of_birth = dgc_name.date_of_birth.clone();
        let mut full_name = firstname.clone();
        full_name.push_str(" ");
        full_name.push_str(&surname);

        self.certificates.insert(
            (firstname.clone(), full_name.clone(), date_of_birth),
            certificate_container,
        );
        Ok((firstname, full_name, signature_validity))
    }
}
