#[derive(serde::Deserialize)]
pub struct Response<T> {
    pub code: String,
    pub message: Option<String>,
    pub data: Option<T>,
}
