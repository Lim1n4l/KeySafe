pub enum Errors  
{
 CreateDataDbError ,
 CreateTableError ,
 StdinError ,
 GenerateKeyError ,
 GetNonceError ,
 CipherError ,
 HashingError ,
 InsertError ,
 SelectError ,
 ShowDatabaseError , 
 ParseError ,
 DeleteFromDatabaseError ,
 EditUsernameError ,
 EditPasswordError ,
 GetEncryptedPasswordError ,
 InvalidId
}


pub static SOMETHING_WRONG:&str = "[!] Something went wrong try again" ;
