use serde::Serialize;

#[derive(Serialize)]
pub enum Status {
    Success,
    Fail,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Success => "success",
            Status::Fail => "fail",
        }
    }

    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
