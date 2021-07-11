use thiserror::Error;


#[derive(Error, Debug)]
pub enum IRError {
    #[error("cannot tranform \"{0}\" into \"{1}\" value type!")]
    BadTranformType(&'static str, &'static str)
}