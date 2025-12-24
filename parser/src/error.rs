use syn::Error as SynError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] SynError),
}
