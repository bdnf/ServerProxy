#[derive(Serialize, Deserialize,Default, Debug)]
pub struct User {
    pub name: String,
    pub group: Option<Group>,
    pub id: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub group_name: String,
    pub allowed_verbs: Vec<String>,
}
