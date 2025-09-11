#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleProjectResponseDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "defaultProject")]
    pub default_project: bool,
}

impl SingleProjectResponseDto {
    pub fn new(id: String, name: String, created_at: String, default_project: bool) -> SingleProjectResponseDto {
        SingleProjectResponseDto {
            id,
            name,
            created_at,
            default_project,
        }
    }
}


