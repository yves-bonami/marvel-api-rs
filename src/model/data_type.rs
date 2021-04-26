use super::series::Series;

#[derive(Debug, Deserialize)]
pub enum DataType{
    Series(Series)
}