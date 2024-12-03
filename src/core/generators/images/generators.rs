use super::base::ImageGenerator;
use barcoders::generators::image::*;
use barcoders::generators::svg::*;
pub struct PNGImageGenerator;

impl ImageGenerator for PNGImageGenerator{
    fn generate(&self, data: &Vec<u8>, height: u32) -> Result<Vec<u8>, &'static str>{
        let img : Image = Image::png(height);
        let out = match img.generate(&data[..]){
          Err(e) => {eprintln!("IMG Gene: {}", e); return Err("No se ha podido generar la imagen")},
          Ok(data) => data
        };
        Ok(out)
    }
}

pub struct WebpImageGenerator;

impl ImageGenerator for WebpImageGenerator{
    fn generate(&self, data: &Vec<u8>, height: u32) -> Result<Vec<u8>, &'static str>{
        let img : Image = Image::webp(height);
        let out = match img.generate(&data[..]){
          Err(e) => { eprintln!("IMG Gene: {}", e); return Err("No se ha podido generar la imagen") },
          Ok(data) => data
        };
        Ok(out)
    }
}

pub struct SvgImageGenerator;

impl ImageGenerator for SvgImageGenerator{
    fn generate(&self, data: &Vec<u8>, height: u32) -> Result<Vec<u8>, &'static str>{
        let img  = SVG::new(height);
        let out = match img.generate(&data[..]){
          Err(e) => {eprintln!("IMG Gene: {}", e); return Err("No se ha podido generar la imagen")},
          Ok(data) => data
        };
        Ok(out.as_bytes().to_vec())
    }
}