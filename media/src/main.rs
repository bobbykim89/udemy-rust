#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
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

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media)  {
        self.items.push(media);
    }
    fn get_by_index (&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // good result
            // MighHaveAValue::ThereIsAValue(&self.items[index])
            return MightHaveAValue::ThereIsAValue(&self.items[index]);
        }
        // bad result
        MightHaveAValue::NoValueAvailable
    }
}

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::AudioBook { title: String::from("An Adventure of Pollito") };
    let good_movie = Media::Movie {
        title: String::from("Harry Pollo and the mystery of disappearing poo"),
        director: String::from("Manguito Lovebird")
    };
    let bad_book = Media::Book { 
        title: String::from("Harry Pollo and the Half Blooded Feathers"), 
        author: String::from("Manguito Lovebird")
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // println!("{}",audiobook.description());
    // println!("{}",good_movie.description());
    // println!("{}",bad_book.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // println!("{:#?}", catalog.items.get(0));
    // match catalog.items.get(0) {
    //     Option::Some(value) => {
    //         println!("Item: {:#?}", value);
    //     },
    //     Option::None => {
    //         println!("Nothing at that index");
    //     }
    // }
    // let item = catalog.get_by_index(40);
    // println!("{:#?}", item);
    match catalog.get_by_index(100) {
        MightHaveAValue::ThereIsAValue(value) => {
            println!("Item: {:#?}", value);
        },
        MightHaveAValue::NoValueAvailable => {
            println!("No value available");
        }
    }
}
