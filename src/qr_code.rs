use qrcodegen::{QrCode, QrCodeEcc};
use std::fs::File;
use std::io::Write;

/*
static BLACK_QR: &'static str = "██";
static WHITE_QR: &'static str = "  ";
*/

pub struct QRString {
    qr_code: QrCode,
}

impl QRString {
    pub fn new(data: &str) -> Result<Self, qrcodegen::DataTooLong> {
        // Error correction level
        let errcorlvl: QrCodeEcc = QrCodeEcc::Low;
        // Make and print the QR Code symbol
        let qr_code = QrCode::encode_text(data, errcorlvl);
        qr_code.map(|qr_code| Self { qr_code })
    }

    // The following function was taken from https://raw.githubusercontent.com/nayuki/QR-Code-generator/master/rust/examples/qrcodegen-demo.rs
    // Returns a string of SVG code for an image depicting
    // the given QR Code, with the given number of border modules.
    // The string always uses Unix newlines (\n), regardless of the platform.
    fn to_svg_string(&self, border: i32) -> String {
        assert!(border >= 0, "Border must be non-negative");
        let mut result = String::new();
        result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
        result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
        let dimension = self
            .qr_code
            .size()
            .checked_add(border.checked_mul(2).unwrap())
            .unwrap();
        result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
        result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
        result += "\t<path d=\"";
        for y in 0..self.qr_code.size() {
            for x in 0..self.qr_code.size() {
                if self.qr_code.get_module(x, y) {
                    if x != 0 || y != 0 {
                        result += " ";
                    }
                    result += &format!("M{},{}h1v1h-1z", x + border, y + border);
                }
            }
        }
        result += "\" fill=\"#000000\"/>\n";
        result += "</svg>\n";
        result
    }
    pub fn write_svg(&self, path: &str) {
        let mut buffer = File::create(path).unwrap();
        write!(buffer, "{}", self.to_svg_string(4));
    }
}

/*
// Allows converting the QR code to a String to print it
impl std::fmt::Display for QRString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let border: i32 = 4;
        for y in -border..self.qr_code.size() + border {
            for x in -border..self.qr_code.size() + border {
                if self.qr_code.get_module(x, y) {
                    result.push_str(WHITE_QR);
                } else {
                    result.push_str(BLACK_QR);
                };
            }
            result.push_str("\n");
        }
        result.push_str("\n");
        write!(f, "{}", result)
    }
}
*/
