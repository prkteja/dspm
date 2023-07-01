# dspm
#### A Dead-Simple CLI Password Management tool for linux
- Passwords are individually encrypted using RSA, base64 encoded and stored in a simple json format
- Keys and passwords are stored under `~/.dspm`
- Supports multiple accounts for each domain and multiple passwords for each account

## Usage
Generate keys and initialize password store
```bash
dspm init 
```
Prompt for new password and add it to password store
```bash
dspm add 'github.com' 'git_username'
```
List all domains
```bash
dspm list
```
List all accounts for domain.com
```bash
dspm list -d 'domain.com'
```
Prompt for master key, decrypt and display password
```bash
dspm show 'github.com' 'git_username'
```
