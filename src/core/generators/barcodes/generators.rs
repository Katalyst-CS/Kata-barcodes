
use super::base::BarcodeGenerator;
use super::base::generate;

use barcoders::sym::*;
use ean13::{EAN13, Bookland, UPCA, JAN};
use ean8::EAN8;
use code11::{Code11, USD8};
use code128::Code128;
use code39::Code39;
use codabar::Codabar;
use code93::Code93;

pub struct EAN13Generator;

impl BarcodeGenerator for EAN13Generator{
    fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
        let barcode:  EAN13 = match EAN13::new(&data) {
          Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
        };
        let bytes = barcode.encode();
        generate(&bytes, heigh, out_img)
    }
}

pub struct UPCAGenerator;

impl BarcodeGenerator for UPCAGenerator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match UPCA::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}

pub struct JanGenerator;

impl BarcodeGenerator for JanGenerator{
  fn generate(&self, data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match JAN::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}


pub struct EAN8Generator;

impl BarcodeGenerator for EAN8Generator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match EAN8::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}

pub struct Code11Generator;

impl BarcodeGenerator for Code11Generator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match Code11::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}


pub struct BooklandGenerator;

impl BarcodeGenerator for BooklandGenerator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match Bookland::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}

pub struct USD8Generator;

impl BarcodeGenerator for USD8Generator {
    fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
        let barcode = match USD8::new(&data) {
          Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
        };
        let bytes = barcode.encode();
        generate(&bytes, heigh, out_img)
    }
}

pub struct CodabarGenerator;

impl BarcodeGenerator for CodabarGenerator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match Codabar::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}


pub struct Code128Generator;

impl BarcodeGenerator for Code128Generator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match Code128::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}

pub struct Code39Generator;

impl BarcodeGenerator for Code39Generator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match Code39::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}

pub struct Code93Generator;

impl BarcodeGenerator for Code93Generator{
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str> {
      let barcode = match Code93::new(&data) {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Error while generate barcode: {}", e);
          return Err("No se pudo parsear el contenido")
        }
      };
      let bytes = barcode.encode();
      generate(&bytes, heigh, out_img)
  }
}