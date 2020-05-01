
use diesel::result;
use std::fmt;

pub enum CrudError{
    DBError(result::Error),
}

// We need this to performs a conversion from diesel::result::Error to MyStoreError
impl From<result::Error> for CrudError {
    fn from(error: result::Error) -> Self {
        CrudError::DBError(error)
    }
}


impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result{
        match self{
            CrudError::DBError(error) => write!(f, "{}", error),
        }
    }
}
