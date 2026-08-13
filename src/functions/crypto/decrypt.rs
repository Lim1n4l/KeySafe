use std::io::
{
 Write , 
 stdin , 
 stdout
} ;
use aes_gcm::
{
 Aes256Gcm , 
 KeyInit , 
 Nonce , 
 aead::
 {
  Aead ,
 }
} ;
use base64::
{
 Engine ,
 engine::general_purpose 
} ;
use crate::functions::errors ;
use crate::db::database_manager::
{
 get_encrypted_password , 
 show_database
} ;


pub fn decrypt(key:&[u8;32]) -> Result<() , errors::Errors>
{
 loop 
 {
  match show_database()
  {
   Ok(_) => () ,
   Err(e) => return Err(e) ,
  } ;
  println!("[i] Input id of the password you want to decrypt") ;
  print!("==> : ") ;
  stdout().flush().unwrap() ;
  let mut input = String::new() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => () ,
   Err(_) => return Err(errors::Errors::StdinError)
  }
  let id:i32 = match input.trim().parse()
  {
   Ok(k) => k ,
   Err(_) => return Err(errors::Errors::ParseError)
  } ;

   let (base_password , base_nonce) = match get_encrypted_password(&id)
   {
    Ok(k) => k ,
    Err(errors::Errors::InvalidId) => 
    {
     println!("[!] Invalid id selected => {id}") ;
     println!("[i] Try again") ;
     continue
    }
    Err(e) => return Err(e) 
   } ;
   let encrypted_password = match general_purpose::STANDARD.decode(&base_password)
   {
    Ok(k) => k ,
    Err(_) => return Err(errors::Errors::CipherError)
   } ;
   let vec_nonce = match general_purpose::STANDARD.decode(&base_nonce)
   {
    Ok(k) => k ,
    Err(_) => return Err(errors::Errors::CipherError)
   } ;
   let nonce = match Nonce::try_from(vec_nonce.as_slice())
   {
    Ok(k) => k ,
    Err(_) => return Err(errors::Errors::CipherError)
   } ;
   let cipher = Aes256Gcm::new((key).into());
   let password = match cipher.decrypt(&nonce, encrypted_password.as_ref())
   {
    Ok(k) =>
    {
     match String::from_utf8(k)
     {
      Ok(k) => k ,
      Err(_) => return Err(errors::Errors::CipherError)
     }
    }
    Err(_) => return Err(errors::Errors::CipherError)
   } ;
   println!("[i] Password is {password}") ;
   return Ok(())
  
 }
}