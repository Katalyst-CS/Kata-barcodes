use super::{base::BarcodeGenerator, generators::*};

pub enum BarcodeGeneratorProxy {
    Code128(Code128Generator),
    Code93(Code93Generator),
    Code39(Code39Generator),
    Codabar(CodabarGenerator),
    EAN13(EAN13Generator),
    EAN8(EAN8Generator),
    USD8(USD8Generator),
    Bookland(BooklandGenerator),
    Code11(Code11Generator),
    JAN(JanGenerator),
    UPCA(UPCAGenerator),
}

pub fn barcode_list() -> Vec<String> {
    vec![
        "code128".to_string(),
        "code93".to_string(),
        "code39".to_string(),
        "code11".to_string(),
        "codabar".to_string(),
        "ean13".to_string(),
        "ean8".to_string(),
        "jan".to_string(),
        "bookland".to_string(),
        "upca".to_string(),
        "usd8".to_string(),
    ]
}

impl BarcodeGeneratorProxy {
    pub fn new(type_gen: &str) -> Self {
        match type_gen.to_lowercase().as_str() {
            "code128" => BarcodeGeneratorProxy::Code128(Code128Generator),
            "code93" => BarcodeGeneratorProxy::Code93(Code93Generator),
            "code39" => BarcodeGeneratorProxy::Code39(Code39Generator),
            "code11" => BarcodeGeneratorProxy::Code11(Code11Generator),
            "codabar" => BarcodeGeneratorProxy::Codabar(CodabarGenerator),
            "ean13" => BarcodeGeneratorProxy::EAN13(EAN13Generator),
            "ean8" => BarcodeGeneratorProxy::EAN8(EAN8Generator),
            "bookland" => BarcodeGeneratorProxy::Bookland(BooklandGenerator),
            "usd8" => BarcodeGeneratorProxy::USD8(USD8Generator),
            "jan" => BarcodeGeneratorProxy::JAN(JanGenerator),
            "upca" => BarcodeGeneratorProxy::UPCA(UPCAGenerator),
            _ => panic!("No se reconoce {}", type_gen),
        }
    }

    pub fn generate(
        &self,
        data: &str,
        height: u32,
        image_type: &str,
    ) -> Result<Vec<u8>, &'static str> {
        match self {
            BarcodeGeneratorProxy::Code128(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::Code93(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::Code39(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::Codabar(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::EAN13(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::EAN8(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::USD8(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::Bookland(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::Code11(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::JAN(gen) => gen.generate(data, height, image_type),
            BarcodeGeneratorProxy::UPCA(gen) => gen.generate(data, height, image_type),
        }
    }
}
