use base64::{engine::general_purpose::STANDARD, Engine};
use log::info;
use rs_sha512::{HasherContext, Sha512Hasher};
use std::hash::Hasher;
use std::io::Error;

pub fn check_sign(sign: String, chain: String) -> Result<bool, Error> {
    // Check if two sign are equals, calculate the SHA512 of the chain and
    // compare with the sign
    let mut signer = Sha512Hasher::default();
    signer.write(chain.as_bytes());
    let _ = signer.finish();
    let byte_result = HasherContext::finish(&mut signer);
    let sig_b64 = STANDARD.encode(byte_result);
    info!("Checking sign: {}", sig_b64);
    let equals = sig_b64 == sign;
    Ok(equals)
}

pub fn generate_sign(chain: String) -> Result<String, Error> {
    let mut signer = Sha512Hasher::default();
    signer.write(chain.as_bytes());
    let _ = signer.finish();
    let bytes = HasherContext::finish(&mut signer);
    Ok(STANDARD.encode(bytes))
}
