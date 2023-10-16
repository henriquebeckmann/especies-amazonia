use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{post, HttpResponse, Responder};

#[derive(MultipartForm, Debug)]
pub struct Post {
    pub title: Text<String>,
    pub popular_name: Option<Text<String>>,
    pub description: Option<Text<String>>,
    pub file: TempFile,
    pub date_picture: Option<Text<String>>,
    pub family: Option<Text<String>>,
    pub gender: Option<Text<String>>,
    pub specie: Option<Text<String>>,
    pub location: Text<String>,
    pub locality: Text<String>,
}

#[post("/form")]
pub async fn post(form: MultipartForm<Post>) -> impl Responder {
    println!("{:?}", &form.file);

    HttpResponse::Ok().body("Form sent successfully")
}
