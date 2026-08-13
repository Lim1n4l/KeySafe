use crate::functions::errors ;
use argon2::
{
 Argon2 , 
 password_hash::
 {
  SaltString , 
  rand_core::OsRng  
 }
} ;
use aes_gcm::
{
 Aes256Gcm ,
 Nonce, aead::
 {
  Aead , 
  Generate , 
  KeyInit
 }
} ;



pub fn generate_key(app_password:&str , existing_salt:&Option<SaltString>) -> Result<([u8;32] , SaltString) , errors::Errors>
{
 let salt = match existing_salt
 {
  Some(salt) => salt.clone() ,
  None => SaltString::generate(&mut OsRng)
 } ;
 let mut key = [0u8;32] ;
 match Argon2::default().hash_password_into
 (app_password.as_bytes(), salt.as_str().as_bytes(), &mut key)
 {
  Ok(_) => return Ok((key , salt)) ,
  Err(_) => return Err(errors::Errors::GenerateKeyError) ,
 } ;
}


pub fn encrypt(password:&str , key:&[u8;32]) -> Result<(Vec<u8> , Vec<u8>) , errors::Errors>
{
 let cipher = Aes256Gcm::new((key).into()) ;
 let nonce = Nonce::generate() ;
 let cipher_text = match cipher.encrypt(&nonce, password.as_bytes())
 {
  Ok(k) => k ,
  Err(_) => return Err(errors::Errors::CipherError) , 
 } ;
 let vec_nonce = nonce.to_vec() ;
 return Ok((cipher_text , vec_nonce)) ;

}


