#[derive(Serialize, Deserialize,Default)]
pub struct User {
    pub name: String,
    pub group: Option<Group>,
    pub id: u32,
}
#[derive(Serialize, Deserialize)]
pub struct Group {
    pub group_name: String,
    pub allowed_verbs: Vec<String>,
}
