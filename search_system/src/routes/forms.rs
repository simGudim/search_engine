use askama::{Template};
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Template)]
#[template(path = "login.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginGet;

#[derive(Debug, Deserialize)]
pub struct LoginForm{
    pub username: String,
    pub password: String
}

#[derive(Template)]
#[template(path = "submit_index.html")]
pub struct SubmitIndex;


#[derive(Debug, Deserialize)]
pub struct DirForm{
    pub dir: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryForm{
    pub query_terms: String
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchGet;

#[derive(Template)]
#[template(path = "something.html")]
pub struct Something;


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