#[derive(Debug, Serialize, Deserialize)]
pub struct GetMeResult {
    pub ok:          bool,
    pub description: Option<String>,
    pub error_code:  Option<i64>,
    pub result:      Option<User>,
    pub parameters:  Option<ResponseParameters>
}