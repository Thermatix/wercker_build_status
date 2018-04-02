
pub type Runs = Vec<Run>;

// only define the structs for the data I want to pull out
#[derive(Debug,Serialize, Deserialize)]
pub struct Run {
    pub status: String,
    pub result: String,
    pub user: User,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct User {
    pub meta: Meta,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Meta {
    pub username: String
}

