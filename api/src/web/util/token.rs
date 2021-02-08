use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

#[derive(Debug)]
pub struct ApiToken {
    pub uid: String,
    pub value: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");
        match token {
            Some(token) => Outcome::Success(ApiToken {
                uid: "zzyongx".to_string(),
                value: token.to_string(),
            }),
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
