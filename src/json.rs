
pub type Runs = Vec<Run>;


#[derive(Debug,Serialize, Deserialize)]
pub struct Meta {
    pub username: String
}

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub meta: Meta,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Run {
    pub status: String,
    pub result: String,
    pub user: User,
}
