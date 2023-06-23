use std::error::Error;
use std::fs;
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;
use openssl::symm::Cipher;
use base64::{Engine as _, engine::general_purpose};
use rpassword;
use dirs;

use crate::constants;

pub fn generate_key_pair(key_size: u32, passphrase: String) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let rsa = Rsa::generate(key_size)?;
    let pkey = PKey::from_rsa(rsa)?;
    let pub_key: Vec<u8> = pkey.public_key_to_pem()?;
    let priv_key: Vec<u8> = pkey.private_key_to_pem_pkcs8_passphrase(Cipher::aes_256_cbc(), 
                                                                     passphrase.as_bytes())?;
    Ok((pub_key, priv_key))
}

pub fn encrypt(key: Vec<u8>, secret: Vec<u8>) -> Result<String, Box<dyn Error>> {
    let pub_key = Rsa::public_key_from_pem(key.as_slice())?;
    let mut ciphertext = vec![b'0'; pub_key.size() as usize];
    let ciphertext_len = pub_key.public_encrypt(secret.as_slice(), 
                                               ciphertext.as_mut_slice(), 
                                               Padding::PKCS1)?;
    Ok(general_purpose::STANDARD_NO_PAD.encode(&ciphertext[..ciphertext_len])) 
}

pub fn decrypt(key: Vec<u8>, encoded_ciphertext: String, passphrase: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let ciphertext = general_purpose::STANDARD_NO_PAD.decode(encoded_ciphertext)?;
    let priv_key = Rsa::private_key_from_pem_passphrase(key.as_slice(), passphrase.as_bytes())?;
    let mut secret = vec![b'0'; priv_key.size() as usize];
    let secret_len = priv_key.private_decrypt(ciphertext.as_slice(), 
                                              secret.as_mut_slice(), 
                                              Padding::PKCS1)?;
    Ok(secret[..secret_len].to_vec())
}

pub fn get_passphrase(confirm: bool, master_key: bool) -> Result<String, Box<dyn Error>> {
    let mut equal = false;
    let mut password = String::new();
    while !equal {
        let mut prompt = "Enter Password: ";
        if master_key {
            prompt = "Enter Master Password: ";
        }
        password = rpassword::prompt_password(prompt)?;
        if !confirm {
            break;
        } else {
            equal = password == rpassword::prompt_password("Confirm password: ")?;
            if !equal {
                println!("Passwords did not match! Try again");
            }
        }
    } 
    Ok(password)
}

pub fn get_keys() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut path = dirs::home_dir().ok_or("Unable to identify home directory")?;
    path.push(constants::LOCAL_DIR);
    path.push(constants::KEY_STORE);
    let keys: Vec<u8> = fs::read(path)?;
    Ok(keys)
}

