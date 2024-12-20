use super::base::ImageGenerator;
use super::generators::*;

pub enum ImageGeneratorProxy {
    Png(PNGImageGenerator),
    Svg(SvgImageGenerator),
    Webp(WebpImageGenerator),
}

impl ImageGeneratorProxy {
    pub fn generate(&self, data: &Vec<u8>, height: u32) -> Result<Vec<u8>, &str> {
        match self {
            ImageGeneratorProxy::Png(generator) => generator.generate(&data, height),
            ImageGeneratorProxy::Svg(generator) => generator.generate(&data, height),
            ImageGeneratorProxy::Webp(generator) => generator.generate(&data, height),
        }
    }

    // Factory method to create the appropriate generator based on the type
    pub fn new(image_type: &str) -> Self {
        match image_type.to_lowercase().as_str() {
            "png" => ImageGeneratorProxy::Png(PNGImageGenerator),
            "svg" => ImageGeneratorProxy::Svg(SvgImageGenerator),
            "webp" => ImageGeneratorProxy::Webp(WebpImageGenerator),
            _ => panic!("Unsupported image format: {}", image_type),
        }
    }

    pub fn list() -> Vec<&'static str> {
        vec!["png", "svg", "webp"]
    }

    pub fn list_as_string() -> Vec<String> {
        vec!["png".to_string(), "svg".to_string(), "webp".to_string()]
    }
}
