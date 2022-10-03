use curl::easy::Easy;
use std::fs::File;
use std::io::Write;

pub fn fetch_public_keys() {
    let url = "https://de.dscg.ubirch.com/trustList/DSC/";
    let body = fetch_url(url);
    let pub_keys: Vec<Result<&str, ()>> = body
        .split(&"\"rawData\": \"")
        .skip(1)
        //.step_by(2)
        .map(|string| {
            let pub_key_len = string.find('"');
            if let Some(len) = pub_key_len {
                Ok(string.split_at(len).0)
            } else {
                Err(())
            }
        })
        .collect();
    write_trusted_keys_to_file(pub_keys)
}

fn write_trusted_keys_to_file(pub_keys: Vec<Result<&str, ()>>) {
    let mut f = File::create("trust_list.txt").expect("Unable to create file");
    for key in &pub_keys {
        if let Ok(pub_key) = key {
            write!(f, "{}\n", pub_key);
        } else {
            println!("Yikes, there was an error with the trust list");
        }
    }
}

fn fetch_url(url: &str) -> String {
    // First write everything into a `Vec<u8>`
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    // Convert it to `String`
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    body
}
