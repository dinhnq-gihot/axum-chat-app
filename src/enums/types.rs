use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DataResponse<U> {
    pub msg: String,
    pub data: Option<U>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericResponse<U> {
    pub status: String,
    pub result: DataResponse<U>,
}
