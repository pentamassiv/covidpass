use dgc::DgcContainer;
use qrcode::{EcLevel, QrCode};
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

    pub fn add_certificate(
        &self,
        cert_path: &std::path::Path,
    ) -> Result<((String, String, String), dgc::DgcContainer), Box<dyn Error>> {
        let raw_certificate_data = read_to_string(cert_path)?;
        // Now we can validate the signature (this returns)
        let (mut certificate_container, signature_validity) =
            dgc::validate(&raw_certificate_data, &self.trust_list)
                .expect("Cannot parse certificate data");

        println!("{:#?}", &certificate_container);

        // Checks the validity of the signature
        match signature_validity {
            dgc::SignatureValidity::Valid => println!("The certificate signature is Valid!"),
            e => println!("Could not validate the signature: {}", e),
        }

        // you can call `expand_values()` to resolve all the IDs against a well known valueset embedded in the library
        certificate_container.expand_values();

        println! {"{:?}",certificate_container};
        let dgc_name = certificate_container.certs.get(&1).unwrap();
        let forename = dgc_name.name.forename.clone().unwrap_or("".into());
        let surname = dgc_name.name.surname.clone().unwrap_or("".into());
        let mut full_name = forename.clone();
        full_name.push_str(" ");
        full_name.push_str(&surname);

        Self::generate_qr_code(&raw_certificate_data)?;
        Ok(((forename, surname, full_name), certificate_container))
    }
    fn generate_qr_code(data: &String) -> Result<(), Box<dyn Error>> {
        let qr_code = QrCode::with_error_correction_level(data, EcLevel::L)?;
        let image = qr_code.render::<image::Luma<u8>>().build();
        image.save("/tmp/qrcode.png")?;
        Ok(())
    }
}
