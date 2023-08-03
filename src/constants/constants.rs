// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";
pub const MESSAGE_BAD_REQUEST: &str = "Bad Request";

// Headers
pub const AUTHORIZATION: &str = "Authorization";


// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 3] = ["/api/check", "/api/auth/signup", "/api/auth/login"];

// Default number of items per page
pub const DEFAULT_PER_PAGE: i64 = 10;

// Default page number
pub const DEFAULT_PAGE_NUM: i64 = 1;

pub const EMPTY_STR: &str = "";

pub static KEY: [u8; 16] = *include_bytes!("../secret.key");