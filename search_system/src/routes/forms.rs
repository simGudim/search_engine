use askama::{Template};
use serde::{Deserialize};
use validator::Validate;


#[derive(Template)]
#[template(path = "login.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct Login;

#[derive(Template)]
#[template(path = "something.html")]
pub struct Something;

#[derive(Template)]
#[template(path = "search.html")]
pub struct Search;

#[derive(Debug, Deserialize)]
pub struct LoginForm{
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct DirForm{
    pub dir: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min = 3))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3))]
    pub password: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfile {
    pub full_name: Option<String>,
    pub bio: Option<String>,
    #[validate(url)]
    pub image: Option<String>
}