use std::collections::HashMap;
use std::{fs, path::PathBuf, error::Error};
use serde_json;
use super::db::{self, JsonDb};
use super::super::crypt;

pub struct DBParser {
    encrypted_file: bool,
    filepath: PathBuf,
    json_db: db::JsonDb
}

fn parse_json(path: PathBuf) -> Result<db::JsonDb, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let json_db: db::JsonDb = serde_json::from_str(data.as_str())?;
    Ok(json_db)
}

fn write_json(path: PathBuf, db: &JsonDb) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string(db)?;
    fs::write(path, data)?;
    Ok(())
}

impl DBParser {
    pub fn new(filepath: PathBuf, encrypted_file: bool) -> Result<Self, Box<dyn Error>> {
        let json_db = parse_json(filepath.clone())?;
        Ok( Self {
            filepath,
            encrypted_file,
            json_db
        })
    }
    
    pub fn list_domains(&self) -> Vec<String> {
        let mut domain_list = Vec::new();
        for (key, _) in self.json_db.domains.iter() {
            domain_list.push(key.clone());
        }
        domain_list
    }

    pub fn list_accounts(&self, domain_name: String) -> Option<Vec<String>> {
        self.json_db.domains.get(&domain_name).map(|domain| {
            let mut account_list = Vec::new();
            for (key, _) in domain.iter() {
                account_list.push(key.clone());
            }
            account_list
        })
    }

    pub fn get_password(&self, domain_name: String, user_name: String) -> 
        Result<Option<Vec<u8>>, Box<dyn Error>> {
        let domain = self.json_db.domains.get(&domain_name);
        match domain {
            None => Ok(None),
            Some(dom) => {
                match dom.get(&user_name) {
                    None => Ok(None),
                    Some(account) => {
                        match account.last() {
                            None => Ok(None),
                            Some(password) => {
                                let passphrase = crypt::get_passphrase(false, true)?;
                                let keys = crypt::get_keys()?;
                                Ok(Some(crypt::decrypt(keys, password.clone(), passphrase)?))
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_password(&mut self, domain_name: String, user_name: String, 
                        password: String) -> Result<(), Box<dyn Error>> {
        let encoded_ciphertext = crypt::encrypt(crypt::get_keys()?, password.as_bytes().to_vec())?;
        let domain_exists = self.json_db.domains.contains_key(&domain_name);
        if domain_exists {
            let account_exists = self.json_db.domains.get(&domain_name).unwrap().contains_key(&user_name);
            if account_exists {
                self.json_db.domains.get_mut(&domain_name).unwrap()
                    .get_mut(&user_name).unwrap().push(encoded_ciphertext);
            } else {
                let mut account = Vec::new();
                account.push(encoded_ciphertext);
                self.json_db.domains.get_mut(&domain_name).unwrap().insert(user_name, account);
            }
        } else {
            let mut account = Vec::new();
            account.push(encoded_ciphertext);
            let mut domain = HashMap::new();
            domain.insert(user_name, account);
            self.json_db.domains.insert(domain_name, domain);
        }
        write_json(self.filepath.to_owned(), &self.json_db)?;
        Ok(())
    }

}
