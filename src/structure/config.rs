use serde::{Serialize, Deserialize};

// Define structs to explain config.yml structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Color {
    primary: String,
    secondary: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Link {
    name: String,
    url: String,
    icon: Option<String>,
    description: Option<String>,
    color: Option<Color>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    title: String,
    subtitle: String,
    links: Vec<Link>
}

impl Config {
    pub fn new() -> Config {
        Config {title: String::new(), subtitle: String::new(), links: Vec::new()}
    }
}