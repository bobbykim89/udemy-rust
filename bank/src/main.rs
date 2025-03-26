#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,

}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

impl  Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0
        }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    // let account = Account::new(1, String::from("Pollito"));
    // println!("{:#?}", bank);
    // print_account(account);
    let other_bank = bank;
    let bank_2 = Bank::new();
    println!("{:#?}", other_bank);
    println!("{:#?}", bank_2);
}
