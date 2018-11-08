//static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Number: <input type=\"text\" name=\"number\"><br><input type=\"submit\"></body></html>";
pub static NOTFOUND: &[u8] = b"Not Found";
//static URL: &str = "http://127.0.0.1:1337/web_api";
pub static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
pub static LOWERCASE: &[u8] = b"i am a lower case string";
pub static MISSING: &[u8] = b"Missing field";
pub static NOTNUMERIC: &[u8] = b"Number field is not numeric";

pub const HOMEPAGE: &'static str  = "/";
pub const TESTPAGE: &'static str  = "/test";
pub const LOGINPAGE: &'static str  = "/login";
pub const SIGN_IN: &'static str  = "/api/auth/signin";
pub const API_PAGE: &'static str  = "/web_api";

pub const ROUTES: [&'static str; 5] = [HOMEPAGE, TESTPAGE, LOGINPAGE, SIGN_IN, API_PAGE];
