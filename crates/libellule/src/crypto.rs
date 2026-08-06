use aes::Aes128;
use aes::cipher::{BlockModeDecrypt, block_padding};
use aes::cipher::{BlockModeEncrypt, KeyIvInit, block_padding::Pkcs7};
use cbc::{Decryptor, Encryptor};

type Aes128CbcEnc = Encryptor<Aes128>;
type Aes128CbcDec = Decryptor<Aes128>;

pub fn aes_encrypt(plaintext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec::<Pkcs7>(plaintext)
}

pub fn aes_decrypt(
    ciphertext: &[u8],
    key: &[u8; 16],
    iv: &[u8; 16],
) -> Result<Vec<u8>, block_padding::Error> {
    Aes128CbcDec::new(key.into(), iv.into()).decrypt_padded_vec::<Pkcs7>(ciphertext)
}
