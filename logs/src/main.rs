use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);
    println!("Hello, world!");
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
        Ok(..) => print!("Email is valid"),
        Err(reasion_this_failed_validation) => {
            println!("{}", reasion_this_failed_validation)
        }
    }
}

fn validate_email (email: String) -> Result<(), Error> {
    if email.contains("@") {
        // success!
        Ok(())
    } else {
        Err(Error::other("Emails must have an @"))
    }
}


fn divide (a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0."))
    } else {
        Ok(a / b)
    }
}