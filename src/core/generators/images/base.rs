pub trait ImageGenerator{
  fn generate(&self, data: &Vec<u8>, height: u32) -> Result<Vec<u8>, &'static str>;
}