use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct UserToken{
	pub id: i64,
	pub email:String,
	pub iat: i64,
	pub exp: i64
}