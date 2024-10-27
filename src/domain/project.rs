use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Project {
    name: String,
    slug: String,
    short_description: String,
    card_background_image: String,
}

impl Project {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_slug(&self) -> &str {
        &self.slug
    }

    pub fn get_short_description(&self) -> &str {
        &self.short_description
    }

    pub fn get_card_background_image(&self) -> &str {
        &self.card_background_image
    }
}
