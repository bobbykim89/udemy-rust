use std::fs;
use std::io::Error;

fn main() {
    // let text = fs::read_to_string("logs.txt");
    // println!("{:#?}", text);

    match divide(5.0, 0.0) {
        Ok(result_of_difision) => {
            println!("{}", result_of_difision)
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong)
        }
    }
    println!("{:#?}", divide(5.0, 3.0));

    match validate_email(String::from("asdf@gmail.com")) {
        Ok(..) => print!("Email is valid\n"),
        Err(reasion_this_failed_validation) => {
            println!("{}", reasion_this_failed_validation)
        }
    }

    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        // String::from("OLives"),
    ];

    match validate_ingredients(&ingredients) {
        Ok(..) => println!("Ingredients are good to go\n"),
        Err(error) => println!("Failed validation: {}\n", error)
    }

    // let mut error_logs = vec![];

    // match fs::read_to_string("logs.txt") {
    //     Ok(text_that_was_read) => {
    //         // println!("{}", text_that_was_read.len())
    //         let error_logs = extract_errors(text_that_was_read.as_str());
    //         // println!("{:#?}", error_logs);
    //         // error_logs = extract_errors(text_that_was_read.as_str());
    //         // writing data to a file:
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("write errors.txt"),
    //             Err(reason_write_fialed) => {
    //                 println!("Writing of errrors.txt filed: {}", reason_write_fialed)
    //             }
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("Failed to read file: {}", why_this_failed)
    //     }
    // }
    // println!("{:#?}", error_logs);
    let text = fs::read_to_string("logs.txt").expect("faied to read logs.txt");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt");
}

fn validate_email (email: String) -> Result<(), Error> {
    if email.contains("@") {
        // success!
        Ok(())
    } else {
        Err(Error::other("Emails must have an @"))
    }
}

fn validate_ingredients (ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("Too many ingredients."))
    } else {
        Ok(())
    }
}


fn divide (a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0."))
    } else {
        Ok(a / b)
    }
}

/**
* @Stack: Fast but limited size (2-8MB)
* @Heap: Slow, but can grow to store a lot of data
* @Data: Stores literal values that we write into out code
* @Why is there &String and &str? Both provide a read-only reference to text data
*   reason #1: &str lets you refer to text in the data segment without a heap allocation
*   reason #2: &str lets you 'slice' (take a portion) of text that is already on the heap
* String:
*   - Use anytime we want ownership of text
*   - Use anytime we want text that can grow or shrink
*   - Use memory in: Stack and Heap
* &String:
*   - Rarely used
*   - Note: Rust will automatically turn &String to &str for you
*   - Use memory in: Stack
* &str:
*   - Use any time you don't want to take ownership of text
*   - Use any time you want to refer to a 'portion' of a string owned by something else
*   - Note: refers directly to heap-allocated or data-allocated text
*   - Use memory in Stack
*/

/**
 * if out function receives some text and we need to return text, should we always return a String?
 *  - Depends!
 *  - Returning a String required extra allocation on the heap
 *  - We would have been fine returning &str if we didn't expect it to have outlive the input text
 */

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}



// fn extract_errors(text: &str) -> Vec<String> {
//     let split_text = text.split("\n");
//     let mut results = vec![];

//     for line in split_text {
//         if line.starts_with("ERROR") {
//             results.push(line.to_string());
//         }
//     }

//     results
// }