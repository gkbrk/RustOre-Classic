extern crate crypto;
use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn is_authenticated(salt: String, username: String, verification_key: String) -> bool{
    let mut md5_hasher = Md5::new();
    md5_hasher.input_str((salt + username.as_slice()).as_slice());
    return md5_hasher.result_str() == verification_key;
}
