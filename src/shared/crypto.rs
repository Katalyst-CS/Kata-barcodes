use std::{hash::{Hash, Hasher}, io::Error};

use base64::{engine::general_purpose::STANDARD, Engine};
use rs_sha512::Sha512Hasher;
pub fn check_sign(sign: String, chain: String ) -> Result<bool, Error>
{
    let mut signer = Sha512Hasher::default();
    signer.write(chain.as_bytes());
    let chain_sign = signer.finish();
    let sig_b64 = STANDARD.encode::<u64>(chain_sign);
    let equals = sig_b64 == sign;
    Ok(equals)
}