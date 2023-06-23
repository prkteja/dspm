use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::error::Error;
use serde_json::json;
use dirs;
use super::crypt;

pub struct Initializer {
    local_dir: PathBuf,
    key_store: String,
    pwd_store: String,
    key_size: u32
}

impl Initializer {
    pub fn new(local_dir: String, key_store: String, pwd_store: String, key_size: u32) -> Self {
        let mut absolute_path = dirs::home_dir().unwrap();
        absolute_path.push(local_dir);
        Self {
            local_dir: absolute_path,
            key_store,
            pwd_store,
            key_size
        }
    }

    pub fn create_local_dir(&self) {
        let result = fs::create_dir(self.local_dir.clone());
        match result {
            Ok(_) => {
                println!("{}", "Created local directory ".to_string() + 
                         self.local_dir.as_path().to_str().unwrap());
            }
            Err(e) => {
                eprintln!("{}", "Unable to create local directory ".to_string() + 
                          self.local_dir.as_path().to_str().unwrap());
                eprintln!("{}", e);
                std::io::stderr().flush().unwrap();
                process::exit(1);
            }
        }
    }

    pub fn generate_and_save_keys(&self) -> Result<(), Box<dyn Error>> {
        let passphrase = crypt::get_passphrase(true, true)?;
        println!("Generating keys ...");
        let (pub_key, priv_key) = crypt::generate_key_pair(self.key_size, passphrase)?;
        let mut filepath = PathBuf::clone(&self.local_dir);
        filepath.push(&self.key_store);
        let mut file = fs::OpenOptions::new().create(true).write(true).open(filepath)?;
        println!("Saving keys ...");
        file.write_all(&pub_key)?;
        file.write_all(b"\n")?;
        file.write_all(&priv_key)?;
        file.flush()?;
        Ok(())
    }

    pub fn create_pwd_db(&self) -> Result<(), Box<dyn Error>> {
        let json_db = json!({
            "version": "0.1",
            "key_size": self.key_size,
            "domains": {}
        });
        let mut filepath = PathBuf::clone(&self.local_dir);
        filepath.push(&self.pwd_store);
        let mut file = fs::OpenOptions::new().create(true).write(true).open(filepath)?;
        println!("Creating password store ...");
        file.write_all(json_db.to_string().as_bytes())?;
        file.flush()?;
        Ok(())
    }

    pub fn initialize(&self) {
        self.create_local_dir();
        match self.generate_and_save_keys() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error generating and saving keys");
                eprintln!("{}", e);
                std::io::stderr().flush().unwrap();
                process::exit(1);
            }
        };
        match self.create_pwd_db() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error creating password store");
                eprintln!("{}", e);
                std::io::stderr().flush().unwrap();
                process::exit(1);
            }
        };
    } 
}
