use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct transformation_types{
    pub Transformation: String,
    pub Type: String,
    pub Name: String,
    pub Column_name: String,
    pub Expression: String
}