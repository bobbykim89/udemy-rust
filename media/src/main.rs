mod content;

use content::media::Media;
use content::catalog::Catalog;

// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable
// }

// fn print_media(media: Media) {
//     println!("{:#?}", media);
// }

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
    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;
    // println!("{:#?}", item);
    // match catalog.get_by_index(1) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     },
    //     None => {
    //         println!("No value available");
    //     }
    // }
    // if let Some(value) = catalog.get_by_index(9999) {
    //     println!("Item in pattern match: {:#?}", value);
    // } else {
    //     println!("No value!");
    // }
    // println!("{:#?}", item.unwrap());
    // println!("{:#?}", item.expect("expected there to be an item here!"));
    println!("{:#?}", item.unwrap_or(&placeholder));
}
