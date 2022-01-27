use super::*;

pub fn load_certificate(
    cert_path: &std::path::Path,
) -> Result<(String, String, greenpass::HealthCert), Box<dyn Error>> {
    let data = read_to_string(cert_path)?;
    let health_cert;
    #[cfg(feature = "crates")]
    {
        health_cert = greenpass_crates::parse(&data)?;
    }

    #[cfg(feature = "local")]
    {
        health_cert = greenpass::parse(&data)?;
    }
    let givenname = health_cert.passes[0].givenname.clone();
    let surname = health_cert.passes[0].surname.clone();
    let mut full_name = givenname.clone();
    full_name.push_str(" ");
    full_name.push_str(&surname);
    generate_qr_code(&data)?;
    Ok((givenname, full_name, health_cert))
}

fn generate_qr_code(data: &String) -> Result<(), Box<dyn Error>> {
    let qr_code = QrCode::with_error_correction_level(data, EcLevel::L)?;
    let image = qr_code.render::<image::Luma<u8>>().build();
    image.save("/tmp/qrcode.png")?;
    Ok(())
}
