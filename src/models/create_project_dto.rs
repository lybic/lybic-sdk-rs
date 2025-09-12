#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProjectDto {
    /// Name of the project.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateProjectDto {
    pub fn new(name: String) -> CreateProjectDto {
        CreateProjectDto { name }
    }
}
