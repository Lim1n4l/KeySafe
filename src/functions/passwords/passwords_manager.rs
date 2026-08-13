use crate::
{
 db::database_manager::
 {
 app_password_check ,
 create_database
 } , 
 functions::
 {
  crypto::encrypt::
  {
   generate_key ,
  } , 
  errors
 }
} ;
use argon2::password_hash::SaltString ;
use rand::RngExt ;
use std::
{
 env , 
 io::
 {
  Write, 
  stdin, 
  stdout
 } , 
 usize
} ;


pub fn is_first_run() -> Result<(Option<[u8;32]> , Option<SaltString>) , errors::Errors>
{
 let current_path = env::current_dir().unwrap() ;
 let database_path = current_path.join("db/data.db") ;
 if ! database_path.exists()   
 {
  println!("===========================") ;
  println!("[i] First time detected") ;
  let app_password = match get_app_password()
  {
   Ok(k) => k ,
   Err(e) => return Err(e) ,
  } ;
  let (key , salt) = match generate_key(&app_password , &None)
  {
   Ok(k) => k ,
   Err(e) => return Err(e) ,
  } ;
  println!("[i] Creating database in db/data.db") ;
  match create_database(&app_password , &salt)
  {
   Ok(_) => return Ok((Some(key) , Some(salt))) ,
   Err(e) => return Err(e) , 
  } ;
 }
 else
 {
  let (login_result , key , salt) = match login()
  {
    Ok(k) => k ,
    Err(e) => return Err(e) ,
  } ;
  if login_result == true 
  {
   return Ok((key , salt)) ;
  }
  else
  {
   return Ok((None , None)) ;  
  }
 }   
}

pub fn get_app_password() -> Result<String , errors::Errors>
{
 println!("[i] Enter the app password") ;
 println!("[i] Dont Forget the password ! or all your encrypted data will be undecryptable !") ; 
 loop
 {
  print!("==> : ") ;
  stdout().flush().unwrap() ;   
  let mut input = String::new() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => return Ok(input.trim().to_string()) ,
   Err(_) => return Err(errors::Errors::StdinError) ,
  } ;
 } ;
}

pub fn login() -> Result<(bool , Option<[u8;32]> , Option<SaltString>) , errors::Errors>
{
 println!("[i] Enter the app password") ;
 let app_password = loop
 {
  print!("==> : ") ;
  stdout().flush().unwrap() ;   
  let mut input = String::new() ;
  match stdin().read_line(&mut input)
  {
   Ok(_) => break input.trim().to_string() ,
   Err(_) => return Err(errors::Errors::StdinError) ,
  } ;
 } ;
 match app_password_check(&app_password)
 {
  Ok((result , salt)) =>
  {
   if result == true
   {
    let (key , _) = match generate_key(&app_password ,&salt)
    {
     Ok(k) => k ,
     Err(e) => return Err(e) , 
    } ;
    return Ok((true , Some(key) , salt)) ;
   }
   else
   {
    return Ok((false , None , None)) ;
   }
  }
  Err(e) => return Err(e) ,
 }
}

pub fn create_strong_password()
{
 loop
 {
  let mut answers = vec![] ;
  let password_length ; 
  loop 
  { 
   println!("====================") ;
   println!("[i] Do you want uppercase ? [y/n]") ;
   print!("==> : ") ;
   stdout().flush().unwrap() ;
   let mut input = String::new() ;
   match stdin().read_line(&mut input)
   {
    Ok(_) =>
    {
     if ! ["y" , "n"].contains(&input.trim())
     {
      println!("[!] Option {} does not exist" , &input.trim()) ;
      println!("[i] Enter a valid option") ;
      continue ;
     }
     else
     {
      answers.push(input.trim().to_string());
      break ;   
     }
    }
    Err(_) => 
    {
     println!("{}" , errors::SOMETHING_WRONG) ;
     continue ;
    } 
   } 
  }
  loop
  {
   println!("[i] Do you want lowercase ? [y/n]") ; 
   print!("==> : ") ;
   stdout().flush().unwrap() ;
   let mut input = String::new() ;
   match stdin().read_line(&mut input)
   {
    Ok(_) => 
    {
     if ! ["y" , "n"].contains(&&input.trim())
     {
      println!("[!] Option {} does not exist" , &input.trim()) ;
      println!("[i] Enter a valid option") ;
      continue ;
     }
     else
     {
      answers.push(input.trim().to_string()) ;
      break ;    
     }
    }
    Err(_) =>
    {
     println!("{}" , errors::SOMETHING_WRONG) ;
     continue ;
    }
   } 
  }
  loop
  {
   println!("[i] Do you want numbers ? [y/n]") ;
   print!("==> : ") ;
   stdout().flush().unwrap() ;
   let mut input = String::new() ;
   match stdin().read_line(&mut input)
   {
    Ok(_) =>
    {
     if ! ["y" , "n"].contains(&&input.trim())
     {
      println!("[!] Option {} does not exist" , &input.trim()) ;
      println!("[i] Enter a valid option") ;
      continue ;     
     }
     else
     {
      answers.push(input.trim().to_string()) ;
      break ;   
     }
    }
    Err(_) =>
    {
     println!("{}" , errors::SOMETHING_WRONG) ;
     continue ;
    }
   }
  }
  loop
  {
   println!("[i] Do you want symbols ? [y/n]") ;
   print!("==> : ") ;
   stdout().flush().unwrap() ;
   let mut input = String::new() ;
   match stdin().read_line(&mut input)
   {
    Ok(_) =>
    {
     if ! ["y" , "n"].contains(&&input.trim())
     {
      println!("[!] Option {} does not exist" , &input.trim()) ;
      println!("[i] Enter a valid option") ;
      continue ;      
     }
     else
     {
      answers.push(input.trim().to_string()) ;
      break ;   
     }
    }
    Err(_) =>
    {
     println!("{}" , errors::SOMETHING_WRONG) ;
     continue ;    
    }
   }    
  }
  loop 
  {
   println!("[i] Enter the password length") ;
   print!("==> : ") ;
   stdout().flush().unwrap() ;
   let mut input = String::new() ;
   match stdin().read_line(&mut input)
   {
    Ok(_) =>
    {
     match input.trim().parse::<usize>()
     {
      Ok(k) =>
      {
       if k == 0 
       {
        println!("[!] Invalid input") ;
        println!("[i] Enter number above 0") ;
        continue ;
       }
       else
       {
        password_length = k ;
        break ;    
       }
      }
      Err(_) => 
      {
       println!("[!] Invalid input") ;
       println!("[i] Enter numbers only") ;
       continue ;
      }
     }
    }
    Err(_) =>
    {
     println!("{}" , errors::SOMETHING_WRONG) ;
     continue ;
    }
    
   }

  }
  let mut charset = String::new() ;
  if answers[0] == "y"
  {
   charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ") ;
  } 
  if answers[1] == "y"
  {
   charset.push_str("abcdefghijklmnopqrstuvwxyz") ;
  }
  if answers[2] == "y"
  {
   charset.push_str("0123456789") ;
  }
  if answers[3] == "y"
  {
   charset.push_str("!@#$%^&*") ;
  }
  if answers.iter().all(|a| a == "n")
  {
   println!("[!] All answers are no") ;
   println!("[i] At least one option must be 'y'") ;
   continue ;
  }
  let charset_length = charset.chars().count() ;
  let mut password = String::new() ;
  let mut rang = rand::rng() ;
  for _ in 0..password_length
  {
   let index = rang.random_range(0..charset_length) ;
   let char = charset.chars().nth(index).unwrap() ;
   password.push(char) ;
  } ;
  print!("password ==> : {password}") ;
  stdout().flush().unwrap() ;
  println!("") ;
  break ;
 }
}

