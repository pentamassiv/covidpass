use super::*;
use dgc::*;

pub fn load_certificate(
    cert_path: &std::path::Path,
) -> Result<((String, String, String), dgc::DgcContainer), Box<dyn Error>> {
    let raw_certificate_data = read_to_string(cert_path)?;
    // This is a X509 certificate that contains a Public Key
    let signature_certificate = "MIIHUTCCBQmgAwIBAgIQUIKkn9dGJ+Fvw2iuEU4tejA9BgkqhkiG9w0BAQowMKANMAsGCWCGSAFlAwQCA6EaMBgGCSqGSIb3DQEBCDALBglghkgBZQMEAgOiAwIBQDBbMQswCQYDVQQGEwJERTEVMBMGA1UEChMMRC1UcnVzdCBHbWJIMRwwGgYDVQQDExNELVRSVVNUIENBIDItMiAyMDE5MRcwFQYDVQRhEw5OVFJERS1IUkI3NDM0NjAeFw0yMTA2MTExNDUwMjFaFw0yMzA2MTUxNDUwMjFaMIHrMQswCQYDVQQGEwJERTEdMBsGA1UEChMUUm9iZXJ0IEtvY2gtSW5zdGl0dXQxJDAiBgNVBAsTG0VsZWt0cm9uaXNjaGVyIEltcGZuYWNod2VpczEdMBsGA1UEAxMUUm9iZXJ0IEtvY2gtSW5zdGl0dXQxDzANBgNVBAcTBkJlcmxpbjEOMAwGA1UEEQwFMTMzNTMxFDASBgNVBAkTC05vcmR1ZmVyIDIwMRkwFwYDVQRhExBEVDpERS0zMDIzNTMxNDQ1MRUwEwYDVQQFEwxDU00wMjY0NjAwMjYxDzANBgNVBAgTBkJlcmxpbjBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABIfd+CIjArF6fe+6Q3hu1otdUvrhhqHpup0jW7QiC3Ek+PvxahpSzgSbyGT0od4Ux8dle1fty86oukdnWoTp6P6jggLpMIIC5TAfBgNVHSMEGDAWgBRxEDKudHF7VI7x1qtiVK78PsC7FjAtBggrBgEFBQcBAwQhMB8wCAYGBACORgEBMBMGBgQAjkYBBjAJBgcEAI5GAQYCMIH+BggrBgEFBQcBAQSB8TCB7jA3BggrBgEFBQcwAYYraHR0cDovL2QtdHJ1c3QtY2EtMi0yLTIwMTkub2NzcC5kLXRydXN0Lm5ldDBCBggrBgEFBQcwAoY2aHR0cDovL3d3dy5kLXRydXN0Lm5ldC9jZ2ktYmluL0QtVFJVU1RfQ0FfMi0yXzIwMTkuY3J0MG8GCCsGAQUFBzAChmNsZGFwOi8vZGlyZWN0b3J5LmQtdHJ1c3QubmV0L0NOPUQtVFJVU1QlMjBDQSUyMDItMiUyMDIwMTksTz1ELVRydXN0JTIwR21iSCxDPURFP2NBQ2VydGlmaWNhdGU/YmFzZT8wcAYDVR0gBGkwZzAJBgcEAIvsQAEBMFoGCysGAQQBpTQCgRYFMEswSQYIKwYBBQUHAgEWPWh0dHA6Ly93d3cuZC10cnVzdC5uZXQvaW50ZXJuZXQvZmlsZXMvRC1UUlVTVF9DU01fUEtJX0NQUy5wZGYwgfAGA1UdHwSB6DCB5TCB4qCB36CB3IZpbGRhcDovL2RpcmVjdG9yeS5kLXRydXN0Lm5ldC9DTj1ELVRSVVNUJTIwQ0ElMjAyLTIlMjAyMDE5LE89RC1UcnVzdCUyMEdtYkgsQz1ERT9jZXJ0aWZpY2F0ZXJldm9jYXRpb25saXN0hjJodHRwOi8vY3JsLmQtdHJ1c3QubmV0L2NybC9kLXRydXN0X2NhXzItMl8yMDE5LmNybIY7aHR0cDovL2Nkbi5kLXRydXN0LWNsb3VkY3JsLm5ldC9jcmwvZC10cnVzdF9jYV8yLTJfMjAxOS5jcmwwHQYDVR0OBBYEFPdFwMQGQturw7wXqRcebaB+nz6vMA4GA1UdDwEB/wQEAwIGwDA9BgkqhkiG9w0BAQowMKANMAsGCWCGSAFlAwQCA6EaMBgGCSqGSIb3DQEBCDALBglghkgBZQMEAgOiAwIBQAOCAgEA0XRsfatBY5BT/HfWbUQvQXtisS0xp2qxVXZkgejDV2r90KsxGAAM9MZIw3eebywbg7ygwhdKwu4ZYGdFpr/cYH+j5pRPCjYoJOsjCEDA7GDtdWenQruM0JcKI4KGgtm01LZGT1L3rBHKh0Dg9fOrAS3L05ZR+cQ1oDKrMUuGm57CDTgcPXmxawmxadjLKOagfPpOkMmZabNMDd3+06gIQ7KfH3BWwZHYbqkg5dHHyb6cvdwQarfLjPKlcWrACWX6KyvYYv8Aj3fxu9w1RYCA1HqOGfIWl0in1LoCJhYLNb4GcS3feqhvnUAKL0BZN+5oSgMbQfi4hqGcwhBKNH4UVfnL47f7dsJxr3ob9U+VLkCUJ7yC6FPjY0pefGgsgQl+9rabBjNaSLwjxoOmEx7PbnjU5xmxipYa7LK+gLuLfZysu+VAFUXAxSFljz5cKWn80sqgSchzOlJRobCI/xTrziQc74pRGV+eWBXpybRu6bvZ8Y96EHpbdyWsG9sDPCeJgiq5XdWfR3JUprVsQzaEBDzBGq1Y0fWxOOoi+gQIQXPvx9sBjc9fdOvnFEODI1NXotab2LlKztNTQU3eEBhbsHjg30a1pIVwiL/nmxsMLxxiNJAZ/9t0D71llbrSBS665BTkmObNnr1xHoS94wg4L9UfinLFsSkHpTer+cQhrA4=";

    // We create a new Trustlist (container of "trusted" public keys)
    let mut trustlist = dgc::TrustList::default();
    // We add the public key in the certificate to the trustlist
    trustlist
        .add_key_from_certificate(signature_certificate)
        .expect("Failed to add key from certificate");

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
