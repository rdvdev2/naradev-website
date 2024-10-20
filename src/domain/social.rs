use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Social {
    name: String,
    link: String,
}

impl Social {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_link(&self) -> &str {
        &self.link
    }

    pub fn get_id(&self) -> String {
        self.name.to_lowercase().replace('-', "")
    }

    pub fn get_logo_url(&self) -> String {
        format!("assets/images/socials/{}.svg", self.get_id())
    }
}
