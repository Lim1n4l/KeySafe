use crate::functions::
{
 crypto::encrypt::encrypt , 
 errors::Errors ,
} ;
use argon2::
{
 PasswordHasher , 
 password_hash::SaltString
} ;
use rusqlite::
{
 Connection , 
 params
} ;
use argon2::Argon2 ;
use base64::
{
 Engine ,
 engine::general_purpose ,
} ;



pub fn create_database(app_password:&str , salt:&SaltString) -> Result<() , Errors>
{
 let connection = match Connection::open("db/data.db")
 {
  Ok(k) => k ,
  Err(_) => return Err(Errors::CreateDataDbError) ,
 } ;
 match connection.execute("CREATE TABLE info
 (
 id INTEGER PRIMARY KEY ,
 username text ,
 password text ,
 nonce text 
 ) ; "
 , [])
 {
  Ok(_) => () ,
  Err(_) => return Err(Errors::CreateTableError) ,
 } ;
 match connection.execute("CREATE TABLE settings
 (
 id INTEGER PRIMARY KEY ,
 app_password text ,
 salt text 
 ) ;", [])
 {
  Ok(_) => () ,
  Err(_) => return Err(Errors::CreateTableError) ,
 } ;
 let hasher = Argon2::default() ;
 let hash = match hasher.hash_password(app_password.as_bytes(), salt)
 {
  Ok(k) => k.to_string() ,
  Err(_) => return Err(Errors::HashingError) ,
 } ;
 let string_salt = salt.to_string() ;
 match connection.execute("INSERT INTO settings
 (app_password , salt)
 VALUES(? , ?) ;", [hash , string_salt])
 {
  Ok(_) => () ,
  Err(_) => return Err(Errors::InsertError) ,
 } ;
 return Ok(()) ;
}

pub fn app_password_check(app_password:&str) -> Result<(bool , Option<SaltString>) , Errors>
{
 let connection = match Connection::open("db/data.db")
  {
   Ok(k) => k ,
   Err(_) => return Err(Errors::CreateDataDbError) ,
  }  ; 
 let salt:SaltString = match connection.query_row("SELECT salt FROM settings WHERE id = 1 ;", [], |row|row.get::<_, String>("salt"))
 {
  Ok(k) => match SaltString::from_b64(&k)
  {
   Ok(k) => k ,
   Err(_) => return Err(Errors::SelectError) ,
  }
  Err(_) => return Err(Errors::SelectError) ,
 } ;
 let password_hash:String = match connection.query_row("SELECT app_password FROM settings", [], |row|row.get("app_password"))
 {
  Ok(k) => k ,
  Err(_) => return Err(Errors::SelectError) ,
 } ;
 let hasher = Argon2::default() ;
 let password_hash_to_check = match hasher.hash_password(app_password.as_bytes() , &salt)
 {
  Ok(k) => k.to_string() ,
  Err(_) => return Err(Errors::HashingError) ,
 } ;
 if password_hash_to_check == password_hash
 {
  return Ok((true , Some(salt))) ;
 }
 else
 {
  return Ok((false , None)) ;   
 }
}

pub fn add_to_database(username:String , encrypted_password:String , vec_nonce:String) -> Result<() , Errors>
{
 let connection = match Connection::open("db/data.db")
  {
   Ok(k) => k ,
   Err(_) => return Err(Errors::CreateDataDbError) ,
  } ;
 match connection.execute("INSERT INTO info
 (username , password , nonce)
 VALUES (? , ? , ?) ;" , [username , encrypted_password , vec_nonce])
 {
  Ok(_) => () ,
  Err(_) => return Err(Errors::InsertError) ,
 }
 return Ok(()) ;  
}

pub fn show_database() -> Result<usize, Errors>
{
 let connection = match Connection::open("db/data.db")
 {
  Ok(connection) => connection,
  Err(_) => return Err(Errors::ShowDatabaseError),
 } ;
 let mut statement = match connection.prepare("SELECT id, username, password, nonce FROM info")
 {
  Ok(statement) => statement,
  Err(_) => return Err(Errors::ShowDatabaseError),
 } ;
 let mut rows = match statement.query([])
 {
  Ok(rows) => rows,
  Err(_) => return Err(Errors::ShowDatabaseError),
 } ;
 let mut row_count = 0;
 while let Some(row) = match rows.next()
 {
  Ok(row) => row,
  Err(_) => return Err(Errors::ShowDatabaseError),
 }
 {
  let id: i64 = match row.get(0)
  {
   Ok(value) => value,
   Err(_) => return Err(Errors::ShowDatabaseError),
  } ;
  let username: String = match row.get(1)
  {
   Ok(value) => value,
   Err(_) => return Err(Errors::ShowDatabaseError),
  } ;
  let password: String = match row.get(2)
  {
   Ok(value) => value,
   Err(_) => return Err(Errors::ShowDatabaseError),
  } ;
  let nonce: String = match row.get(3)
  {
   Ok(value) => value,
   Err(_) => return Err(Errors::ShowDatabaseError),
  } ;
  println!("{id}\t{username}\t{password}\t{nonce}") ;
  row_count += 1;
 }
 Ok(row_count)
}

pub fn delete_from_database(id:&i32) -> Result<usize , Errors>
{
 let connection = match Connection::open("db/data.db")
  {
   Ok(k) => k ,
   Err(_) => return Err(Errors::ShowDatabaseError) ,
  } ;
 match connection.execute("DELETE FROM info WHERE id = ?", [id])
 {
  Ok(k) => return Ok(k) ,
  Err(_) => return Err(Errors::DeleteFromDatabaseError) 
 }     
}

pub fn edit_database(id:&i32 , username:&str , password:&str , key:&[u8;32]) -> Result<Option<usize> , Errors>
{
 let connection = match Connection::open("db/data.db")
  {
   Ok(k) => k ,
   Err(_) => return Err(Errors::ShowDatabaseError) ,
  } ; 
 match (username , password)
 {
  ("*" , "*") => return Ok(None) , 
  (username, "*") => 
  {
   match connection.execute("UPDATE info SET username = ? WHERE id = ?", params![username , id])
   {
    Ok(k) => return Ok(Some(k)) ,
    Err(_) => return Err(Errors::EditUsernameError)
   }
  }
  ("*" , password) => 
  {
   let (vec_encrypted_password , vec_nonce) = match encrypt(password, key)
   {
    Ok(k) => k ,
    Err(e) => return Err(e) 
   } ;
   let encrypted_password = general_purpose::STANDARD.encode(vec_encrypted_password) ;
   let nonce = general_purpose::STANDARD.encode(vec_nonce) ;
   match connection.execute("UPDATE info SET password = ? , nonce = ? WHERE id = ?", params![encrypted_password , nonce , id])
   {
    Ok(k) => return Ok(Some(k)) ,
    Err(_) => return Err(Errors::EditPasswordError)
   }
  }
  (username , password) => 
  {
   let (vec_encrypted_password , vec_nonce) = match encrypt(password, key)
   {
    Ok(k) => k ,
    Err(e) => return Err(e) 
   } ;
   let encrypted_password = general_purpose::STANDARD.encode(vec_encrypted_password) ;
   let nonce = general_purpose::STANDARD.encode(vec_nonce) ;   
   match connection.execute("UPDATE info SET username = ?  , password = ? , nonce = ? WHERE id = ?", params![username , encrypted_password , nonce , id])
   {
    Ok(k) => return Ok(Some(k)) ,
    Err(_) => return Err(Errors::EditUsernameError)
   }
  }
 }
}

pub fn get_encrypted_password(id:&i32) -> Result<(String , String) , Errors>
{
 let connection = match Connection::open("db/data.db")
  {
   Ok(k) => k ,
   Err(_) => return Err(Errors::ShowDatabaseError) ,
  } ; 
 let password = match connection.query_row("SELECT password FROM info WHERE id = ?", params![id] , |row| row.get(0))
 {
  Ok(k) => k  ,
  Err(rusqlite::Error::QueryReturnedNoRows) => return Err(Errors::InvalidId) ,
  Err(_) => return Err(Errors::GetEncryptedPasswordError)
 } ;
 let nonce = match connection.query_row("SELECT nonce FROM info WHERE id = ? ", params![id], |row| row.get(0))
 {
  Ok(k) => k ,
  Err(rusqlite::Error::QueryReturnedNoRows) => return Err(Errors::InvalidId) ,  
  Err(_) => return Err(Errors::GetNonceError) 
 } ;
 return Ok((password , nonce))
}
