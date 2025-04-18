#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    pub fn description(&self) -> String {
        // if let Media::Book {title, author} = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie {title, director} = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::AudioBook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            },
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            },
            Media::AudioBook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast: {}", episode_number)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}