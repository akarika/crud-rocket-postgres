use diesel::result;
use std::fmt;
use rocket::response::Responder;
use rocket_okapi::response::OpenApiResponder;
use okapi::openapi3::Responses;
use rocket_okapi::util::add_schema_response;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::OpenApiError;


#[derive(Responder, Debug)]
pub enum CrudError {
    #[response(status = 500, content_type = "json")]
   // DBError(result::Error),
    DBError(String)
}

type Result<T> = std::result::Result<T, OpenApiError>;
type ResultOpenApiResponder = Result<Responses> ;

//we need to add this trait to documentation CrudError
impl OpenApiResponder<'_> for CrudError {
    fn responses(gen: &mut OpenApiGenerator) -> ResultOpenApiResponder {
        let mut responses = Responses::default();
        let schema = gen.schema_generator().schema_for_any();
        add_schema_response(&mut responses, 500, "application/json", schema.into())?;
        Ok(responses)
    }
}


// We need this to performs a conversion from diesel::result::Error to CrudError
impl From<result::Error> for CrudError {
    fn from(error: result::Error) -> Self {
        CrudError::DBError(error.to_string())
    }
}


impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrudError::DBError(error) => write!(f, "{}", error),
        }
    }
}


/*
impl CrudError {
    fn wrap_body(message: &str) -> Json<ApiErrorMessage> {
        Json(ApiErrorMessage {
            status: "error".to_string(),
            message: CrudError::DBError(error).to_string(),
        })
    }

    pub fn unavailable(message: &str) -> Self {
        Self::DBError(Self::wrap_body(message))
    }
}

struct ApiErrorMessage {
    status: String,
    message: String;
}
*/
