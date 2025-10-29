/**
 * Iterators:
 *  - Used to iterate over any kind of data structure
 *  - they are used behind the scenes when you write for a loop
 *  - follow all the same rules of ownership, borrowing, lifetimes
 *  - usethe option enum 
 */

fn print_elements(elements: &[String]) {
    // we usually don't call 'next' on an iteraror manually
    // for element in elements {
    //     println!("{}", element)
    // }
    // elements.iter().for_each(|el| println!("{}", el));
    elements.iter().map(|el| format!("{} {}", el, el)).for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

/**
 * need to store the argument somewhere? -> favor taking ovnership (receive a value)
 * need to do a calculation with the value? -> favor receiving read-only ref
 * need to change the value in some way? -> favor receiving mutable ref
 */

fn main() {
    // let name = "Pollito";
    // println!("Hello, {}!", name);
    let mut colors = vec![
        String::from("lovebird"),
        String::from("conure"),
        String::from("cockatiel")
    ];
    // let mut colors_iter = colors.iter();
    
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());

    // print_elements(&colors[0..2]);
    shorten_strings(&mut colors);
    println!("{:#?}", colors);
}
