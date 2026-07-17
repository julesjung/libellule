use aes::Aes128;
use aes::cipher::{BlockModeEncrypt, KeyIvInit, block_padding::Pkcs7};
use cbc::Encryptor;

type Aes128CbcEnc = Encryptor<Aes128>;

pub fn encode_request_count(request_count: u32, key: &[u8; 16], iv: &[u8; 16]) -> String {
    let plaintext = request_count.to_string();

    let request_count =
        Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec::<Pkcs7>(plaintext.as_bytes());

    hex::encode(request_count)
}
