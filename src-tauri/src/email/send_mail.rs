use lettre::message::{Message,header,Attachment,MultiPart,SinglePart};
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::error::Error;
// use std::f32::consts::E;
use std::fs;

use crate::email::getid::get_mail_pw;

pub struct MailHandeler{
    senderid:String,
    passwd:String,
}
impl MailHandeler{
    pub fn init<'a>()->Result<Self,crate::email::getid::Errors<'a>>{
        let email=get_mail_pw();
        match email {
            Ok(s)=>{
                Ok(MailHandeler{
                    senderid:s.0,
                    passwd:s.1,
                })
            }
            Err(q)=>{
                // todo!("logger")
                Err(q)
            }
        }
    }
    pub fn send_text(&self,to:String,subject:String,body:String)->Result<Response,Box<dyn Error>>{
        let email = Message::builder()
        .from(self.senderid.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body)?;
        let cred=Credentials::new(self.senderid.clone(), self.passwd.clone());
        let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(cred)
        .build();
        Ok(mailer.send(&email)?)
    }
    pub fn send_bin_attachements(&self,to:String,subject:String,body:Option<String>,file_name:String,bin:Vec<u8>)->Result<Response,Box<dyn Error>>{
        let text_part = SinglePart::builder()
        .header(header::ContentType::TEXT_PLAIN)
        .body(match body {
            Some(x)=>x,
            None=>"".to_string()
        });

        let attachment = Attachment::new(file_name)
        .body(bin, "application/octet-stream".parse()?);
        let email = Message::builder()
        .from(self.senderid.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .multipart(
            MultiPart::mixed()
                .singlepart(text_part)
                .singlepart(attachment),
        )?;
        let cred=Credentials::new(self.senderid.clone(),self.passwd.clone());
        let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(cred)
        .build();
        Ok(mailer.send(&email)?)
    }
    pub fn send_file(&self,to:String,subject:String,body:Option<String>,filepath:String)->Result<Response,Box<dyn Error>>{
        let f=fs::read(filepath.clone())?;
        let fp=filepath.split(r"\").into_iter().last().ok_or("path errorx")?;
        Ok(self.send_bin_attachements(to, subject, body, fp.to_string(), f)?)
    }
}