use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Game {
    pub name: String,
    #[serde(rename = "objectID")]
    pub object_id: String,
    pub oslist: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub(crate) struct PostResult {
    pub hits: Vec<Game>
}
