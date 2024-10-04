use std::fs;
#[derive(PartialEq,Debug)]
pub enum Errors<'a>{
    FormatError(&'a str),
    MailNotFound,
    PasskeyNotFound,
    FileError,
    None
}

impl Errors<'_> {
    pub fn is_none(&self)->bool{
        *self==Self::None
    }
    pub fn is_mailnotfound(&self)->bool{
        *self==Self::MailNotFound
    }
    pub fn is_passkeynotfound(&self)->bool{
        *self==Self::PasskeyNotFound
    }
    pub fn is_formaterr(&self)->bool{
        match *self {
            Self::FormatError(_)=>{true}
            _=>{false}   
        }
    }
    pub fn is_fileerr(&self)->bool{
        *self==Self::FileError
    }
}
pub fn get_mail_pw<'a>()->Result<(String,String ),Errors<'a>>{
    let txt=match  fs::read_to_string(r".\email.noedit"){
        Ok(x)=>{x},
        Err(_)=>{
            return Err(Errors::FileError);
        }
    };
    let mut id=None;
    let mut passwd=None;
    let mut err=Errors::None;
    for i in txt.lines(){
        match i {
            x if x[..14]==*"SENDER_MAIL_ID"=>{
                let (s,e)=(i.find("["),i.find("]"));
                if s.is_none() || e.is_none(){
                    err=Errors::FormatError("Error in email.noedit file .Section 'SENDER_MAIL_ID'");
                    break;
                }
                id=Some(i[s.unwrap()+1..e.unwrap()].trim().to_string());
                println!("{:?}",id);
            }
            x if x[..7]==*"PASSKEY"=>{
                let (s,e)=(i.find("["),i.find("]"));
                if s.is_none() || e.is_none(){
                    err=Errors::FormatError("Error in email.noedit file .Section 'SENDER_MAIL_ID'");
                    break;
                }
                passwd=Some(i[s.unwrap()+1..e.unwrap()].trim().to_string());
                println!("{:?}",passwd);
            }
            _=>{}
        }
    }
    if !err.is_none(){
        return Err(err);
    }else if id.is_none() {
        return Err(Errors::MailNotFound);
    }else if passwd.is_none() {
        return Err(Errors::PasskeyNotFound);
    }
    Ok((id.unwrap(),passwd.unwrap()))
}