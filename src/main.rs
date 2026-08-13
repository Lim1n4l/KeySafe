mod style ;
mod functions ;
mod db ;



use std::io::
{
 Write,
 stdin,
 stdout
} ;
use crate::
{ 
 functions::
 {
  crypto::decrypt::decrypt , 
  errors::
  {
   SOMETHING_WRONG ,
   Errors::
   {
    CipherError ,
    CreateDataDbError ,
    CreateTableError ,
    DeleteFromDatabaseError ,
    EditPasswordError ,
    EditUsernameError ,
    GenerateKeyError ,
    GetEncryptedPasswordError ,
    GetNonceError ,
    HashingError ,
    InsertError ,
    ParseError ,
    SelectError ,
    ShowDatabaseError ,
    StdinError ,
   }
  } , 
  passwords::
  {
   entries_manager::
   {
    add_entry , 
    delete_entry , 
    edit_entry 
   } , 
   passwords_manager::
   {
    create_strong_password ,  
    is_first_run 
   } 
  }
 }, style::ui::menu  
} ;


fn main()
{
 let (key , _) = match is_first_run()
 {
  Ok((Some(key) , Some(salt))) =>
   {
    (key , salt)
   }
  Ok((None , None)) | Ok(_) =>
   {
    println!("[!] Invalid login password") ; 
    return ;
   }
  Err(e) => match e
   {
    CreateDataDbError | CreateTableError | InsertError | 
    SelectError | DeleteFromDatabaseError | ShowDatabaseError => 
    {
     println!("[!] There is an error with database") ;
     return ;
    } ,
    GenerateKeyError  => 
    {
     println!("[!] There is an error in generating the key") ;
     return ;
    } ,
    CipherError | HashingError => 
    {
     println!("[!] There is an error in crypto module") ;
     return ;
    } ,
    StdinError | ParseError | _ => 
    {
     println!("{}" , SOMETHING_WRONG) ;
     return ;
    } ,
   }
  } ;
 loop
 {
  menu() ;
  let user_choice:i32 = loop
  {
   print!("==> : ") ;
   stdout().flush().unwrap() ;
   let mut input = String::new() ;
   match stdin().read_line(&mut input)
   {
    Ok(_) => () ,
    Err(_) =>
    {
     println!("{}" , SOMETHING_WRONG) ;
     continue ;
    }
   } ;
   match input.trim().parse() 
   {
    Ok(k) =>
    {
     if !(1..=6).contains(&k)
     {
      println!("[!] Option {k} does not exist") ;
      println!("[i] Enter a valid option") ;
      continue ;
     }
     else 
     {
      break k ;   
     } 
    } 
    Err(_) => 
    {
     println!("[!] Invalid input format") ;
     println!("[i] Input must be an integer not a decimal or text") ; 
     continue ;
    } 
   } ;
  } ;
  if user_choice == 1 
  {
   match add_entry(&key)
   {
    Ok(_) =>
    {
     println!("[+] Username and password added to database") ;
     continue ;
    }
    Err(e) => match e
    {
     CreateDataDbError | CreateTableError | InsertError | SelectError => 
     {
      println!("[!] There is an error with database") ;
      return ;
     } 
     GenerateKeyError => 
     {
      println!("[!] There is an error in generating the key") ;
      return ;
     } 
     CipherError | HashingError => 
     {
      println!("[!] There is an error in crypto module") ;
      return ;
     } 
     StdinError | _ => 
     {
      println!("{}" , SOMETHING_WRONG) ;
      return ;
     } 
    } 
   } 
  }
  if user_choice == 2
  {
   match delete_entry()
   {
    Ok(_) => () ,
    Err(e) =>
    {
     match e
     {
      CreateDataDbError | CreateTableError | InsertError |
      SelectError | DeleteFromDatabaseError | ShowDatabaseError | 
      EditPasswordError | EditUsernameError => 
      {
       println!("[!] There is an error with database") ;
       return ;
      } ,
      GenerateKeyError | GetNonceError => 
      {
       println!("[!] There is an error in generating the key") ;
       return ;
      } ,
      CipherError | HashingError | GetEncryptedPasswordError => 
      {
       println!("[!] There is an error in crypto module") ;
       return ;
      } ,
      StdinError | ParseError => 
      {
       println!("{}" , SOMETHING_WRONG) ;
       return ;
      }
      _ => ()
     }
    }
   }
  } 
  if user_choice == 3
  {
   match edit_entry(&key)
   {
    Ok(_) => () ,
    Err(e) => match e
    {
     CreateDataDbError | CreateTableError | InsertError |
     SelectError | DeleteFromDatabaseError | ShowDatabaseError | 
     EditPasswordError | EditUsernameError | GetEncryptedPasswordError |
     GetNonceError => 
     {
      println!("[!] There is an error with database") ;
      return ;
     } 
     GenerateKeyError => 
     {
      println!("[!] There is an error in generating the key") ;
      return ;
     }
     CipherError | HashingError  => 
     {
      println!("[!] There is an error in crypto module") ;
      return ;
     } 
     StdinError | ParseError => 
     {
      println!("{}" , SOMETHING_WRONG) ;
      return ;
     }  
     _ => ()   
    }
   }
  }
  if user_choice == 4
  {
   match decrypt(&key)
   {
    Ok(_) => () ,
    Err(e) => match e
    {
     CreateDataDbError | CreateTableError | InsertError |
     SelectError | DeleteFromDatabaseError | ShowDatabaseError | 
     EditPasswordError | EditUsernameError => 
     {
      println!("[!] There is an error with database") ;
      return ;
     } 
     GenerateKeyError | GetNonceError => 
     {
      println!("[!] There is an error in generating the key") ;
      return ;
     } 
     CipherError | HashingError | GetEncryptedPasswordError => 
     {
      println!("[!] There is an error in crypto module") ;
      return ;
     } 
     StdinError | ParseError => 
     {
      println!("{}" , SOMETHING_WRONG) ;
      return ;
     }
     _ => ()
    }
   }
  }
  if user_choice == 5
  {
   create_strong_password() ;
   continue ;
  } 
  if user_choice == 6
  {
   break ;
  }
 } ;
}
