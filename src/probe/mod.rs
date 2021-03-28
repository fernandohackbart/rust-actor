use rocket::response::content;
use std::string::String;

#[get("/probe")]
pub fn probe_out() -> content::Plain<String>
{
    content::Plain("HEALTHY".to_string())
}