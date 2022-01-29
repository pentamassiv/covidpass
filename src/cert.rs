use qrcode::{EcLevel, QrCode};
use std::{error::Error, fs::read_to_string};

pub fn load_certificate(
    cert_path: &std::path::Path,
) -> Result<((String, String, String), dgc::DgcContainer), Box<dyn Error>> {
    let raw_certificate_data = read_to_string(cert_path)?;

    let pub_keys = crate::pub_keys::read_file("trust_list.txt");

    // We create a new Trustlist (container of "trusted" public keys)
    let mut trustlist = dgc::TrustList::default();

    for key in pub_keys {
        // We add the public key in the certificate to the trustlist
        trustlist
            .add_key_from_certificate(&key)
            .expect("Failed to add key from certificate");
    }

    // Now we can validate the signature (this returns)
    let (mut certificate_container, signature_validity) =
        dgc::validate(&raw_certificate_data, &trustlist).expect("Cannot parse certificate data");

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

    generate_qr_code(&raw_certificate_data)?;
    Ok(((forename, surname, full_name), certificate_container))
}

fn generate_qr_code(data: &String) -> Result<(), Box<dyn Error>> {
    let qr_code = QrCode::with_error_correction_level(data, EcLevel::L)?;
    let image = qr_code.render::<image::Luma<u8>>().build();
    image.save("/tmp/qrcode.png")?;
    Ok(())
}
