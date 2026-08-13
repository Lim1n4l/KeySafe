use crate::
{
 db::database_manager::
 {
  add_to_database , 
  delete_from_database , 
  edit_database , 
  show_database
 } , 
 functions::
 {
  crypto::encrypt::encrypt ,
  errors
 }
} ;
use std::io::
{
 Write, 
 stdin, 
 stdout
} ;
use base64 ::
{
 Engine ,
 engine::general_purpose ,
} ;


pub fn add_entry(key:&[u8;32]) -> Result<() , errors::Errors>
{
 println!("=========================") ;
 println!("[i] Enter username or email") ;
 print!("==> : ") ;
 stdout().flush().unwrap() ;
 let username = loop
 {
  let mut input = String::new() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => break input.trim().to_string() ,
   Err(_) =>
   {
    println!("[!] Something went wrong try again") ;
    continue ;
   }
  }    
 } ;
 println!("[i] Enter password") ;
 print!("==> : ") ;
 stdout().flush().unwrap() ;
 let password = loop
 {
  let mut input = String::new() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => break input.trim().to_string() ,
   Err(_) =>
   {
    println!("[!] Something went wrong try again") ;
    continue ;
   }
  }  
 } ;
 let (encrypted_password , vec_nonce) = match encrypt(&password , key)
 {
  Ok(k) => k ,
  Err(e) => return Err(e) ,
 } ;
 let encrypted_password_string = general_purpose::STANDARD.encode(&encrypted_password) ;
 let nonce_string = general_purpose::STANDARD.encode(&vec_nonce) ;
 match add_to_database(username , encrypted_password_string , nonce_string)
 {
  Ok(_) => () ,
  Err(e) => return Err(e) ,
 } 
 return Ok(()) ;
}

pub fn delete_entry() -> Result<() , errors::Errors>
{
 loop
 {
  let mut input = String::new() ;
  match show_database()
  {
   Ok(_) => () ,
   Err(e) => return Err(e) 
  } ;
  println!("[i] Enter the id of the entry you want to delete") ;
  print!("==> : ") ;
  stdout().flush().unwrap() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => () ,
   Err(_) => return Err(errors::Errors::StdinError) ,
  }
  let id:i32 = match input.trim().parse()
  {
   Ok(k) => k ,
   Err(_) =>  return Err(errors::Errors::ParseError)
  } ;
  match delete_from_database(&id)
  {
   Ok(k) => match k
   {
    0 =>
    { 
     println!("[!] Invalid id selected => {id}") ;
     println!("[i] Try again") ;
     continue
    }
    _ => ()
   }
   Err(e) => return Err(e)
  }
  println!("[+] Entry deleted successfully") ; 
  break ;      
 }
 return Ok(())
}

pub fn edit_entry(key:&[u8;32]) -> Result<() , errors::Errors>
{
 loop
 {
  match show_database()
  {
   Ok(_) => () ,
   Err(e) => return Err(e) 
  } ;
  println!("[i] Enter the id of the entry you want to edit") ;
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
   Err(_) => 
   {
    println!("[!] Invalid input") ;
    println!("[i] Id must be a number") ;
    continue ;
   }
  } ;
  println!("[i] Enter entry you want to edit like [username/password]") ;
  println!("[i] To skip an entry enter * (e.g. */password)") ; 
  print!("==> : ") ;
  stdout().flush().unwrap() ;
  let mut input = String::new() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => () ,
   Err(_) => return Err(errors::Errors::StdinError)
  }
  if ! input.trim().contains("/")
  {
   println!("[!] Invalid input format") ;
   println!("[i] Use the format [username/password] or * to skip") ;
   continue
  }
  let mut parts = input.trim().splitn(2, "/") ;
  let username = match parts.next()
  {
   Some(value) => value ,
   None => 
   {
   println!("[!] Invalid input format") ;
   println!("[i] Use the format [username/password] or * to skip") ;
   continue
   }
  } ;
  let password = match parts.next()
  {
   Some(value) => value ,
   None => 
   {
   println!("[!] Invalid input format") ;
   println!("[i] Use format [username/password] or * to skip") ;
   continue
   }
  } ;
  match edit_database(&id, &username, &password , key)
  {
   Ok(Some(0)) => 
   {
    println!("[!] Invalid id selected => {id}") ;
    println!("[i] Try again") ;
    continue
   }
   Ok(Some(_)) => println!("[+] Entry edited successfully") ,
   Ok(None) => println!("[i] Nothing was changed") ,
   Err(e) => return Err(e)
  }
  return Ok(())
 } 
}

