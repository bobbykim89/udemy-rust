
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

    fn add_account (&mut self, account: Account) {
        self.accounts.push(account);
    }
    fn total_balance (&self) -> i32{
        self.accounts.iter().map(|account| account.balance).sum()
    }
    fn summary (&self) -> Vec<String>{
        self.accounts
        .iter()
        .map(|account| account.summary())
        .collect::<Vec<String>>()
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

    fn deposit (&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary (&self) -> String {
        format!("{} has balance of {}", self.holder, self.balance)
    }
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Pollito"));
    let mut account_2 = Account::new(2, String::from("Chiquito"));

    account.deposit(500);
    account.withdraw(250);
    account_2.deposit(30);

    // println!("{}", account.summary());

    bank.add_account(account);
    bank.add_account(account_2);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
