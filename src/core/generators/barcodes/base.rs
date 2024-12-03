use crate::core::generators::images::proxy::ImageGeneratorProxy;


pub trait BarcodeGenerator {
  fn generate(&self,data: &str, heigh: u32, out_img: &str) -> Result<Vec<u8>, &'static str>;
}

pub fn generate(data: &Vec<u8>, heigh: u32, img_type: &str) -> Result<Vec<u8>, &'static str>
{
  let image_generator: ImageGeneratorProxy = ImageGeneratorProxy::new(&img_type);
  let result = image_generator.generate(&data, heigh);
  Ok(result.unwrap())
}