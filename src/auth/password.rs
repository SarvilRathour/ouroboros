use bcrypt::BcryptError;
use bcrypt::hash;
use bcrypt::DEFAULT_COST;
use bcrypt::verify;
pub fn hash_password(password:&str)->Result<String,BcryptError>{
    hash(password,DEFAULT_COST+2)
}
pub fn verify_password(password:&str,hash:&str)->Result<bool,BcryptError>{
    verify(password, hash)
}