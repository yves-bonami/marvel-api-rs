use super::data_type::DataType;

#[derive(Debug, Deserialize)]
pub struct DataContainer{
    pub offset: Option<u32>,
    pub limit: Option<u32>,
    pub total: Option<u32>,
    pub count: Option<u32>,
    pub results: Option<DataType>,
}