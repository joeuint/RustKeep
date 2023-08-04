pub mod structs {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Database {
        pub version: String,
        pub root: Root,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Root {
        pub entries: Vec<Entry>,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Entry {
        pub uuid: String,
        pub name: String,
        pub url: String,
        pub username: String,
        pub password: String,
        pub notes: String,
        pub creation: i64,
    }
}

pub mod utils {
    pub mod crypto;
    pub mod database;
    pub mod dbops;
    pub mod generator;
    pub mod random;
}
