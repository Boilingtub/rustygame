use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("This is a error for the rustygame engine")]
    TestError,
}
