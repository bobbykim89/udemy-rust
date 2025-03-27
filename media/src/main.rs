#[derive(Debug)]
enum Media {
    Book { title: String, author: String},
    Movie { title: String, director: String},
    AudioBook { title: String }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::AudioBook { title: String::from("An Adventure of Pollito") };

    print_media(audiobook);
}
