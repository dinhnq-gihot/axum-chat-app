use {
    serde::{
        Deserialize,
        Serialize,
    },
    utoipa::ToSchema,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct DataResponse<U> {
    pub msg: String,
    pub data: Option<U>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct GenericResponse<U> {
    pub status: String,
    pub result: DataResponse<U>,
}
