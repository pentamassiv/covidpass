use qrcodegen::{QrCode, QrCodeEcc};

static BLACK_QR: &'static str = "██";
static WHITE_QR: &'static str = "  ";

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
}

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
