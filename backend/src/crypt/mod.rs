// region:      --- Modules
mod error;
pub mod pwd;
pub mod token;

pub use self::error::{Error, Result};

use hmac::{Hmac, Mac};
use sha2::Sha512;
// endregion:   --- Modules

pub struct EncryptContent {
    pub content: String, // clear content
    pub salt: String,    // clear salt
}

pub fn encrypt_into_b64u(key: &[u8], enc_content: &EncryptContent) -> Result<String> {
    let EncryptContent { content, salt } = enc_content;
    // create HMAC-SHA-512 from key
    let mut hmac_sha512 = Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFailHmac)?;
    // add content
    hmac_sha512.update(content.as_bytes());
    hmac_sha512.update(salt.as_bytes());
    // finalise + b64 encode
    let hmac_result = hmac_sha512.finalize();
    let result_bytes = hmac_result.into_bytes();
    let result = base64_url::encode(&result_bytes);
    Ok(result)
}

// region:      --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rand::RngCore;

    #[test]
    fn test_encrypt_into_b64u_ok() -> Result<()> {
        let mut fixture_key = [0u8; 64]; // 512 bits = 64 bytes
        rand::thread_rng().fill_bytes(&mut fixture_key);
        let fixture_enc_content = EncryptContent {
            content: "hello world".to_string(),
            salt: "some pepper".to_string(),
        };
        // TODO: improve this! weak test
        let fixture_result = encrypt_into_b64u(&fixture_key, &fixture_enc_content)?;
        // exec
        let result = encrypt_into_b64u(&fixture_key, &fixture_enc_content)?;
        // check
        assert_eq!(result, fixture_result);
        Ok(())
    }
}
// endregion:   --- Tests
