use serde_json::Error;

pub trait ApiResponse: Sized {
    fn from_json(json: &str) -> Result<Self, Error>;
}
