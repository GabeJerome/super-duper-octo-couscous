mod banking;
mod institution;

use crate::banking::account::Account;
use banking::checking_account::CheckingAccount;
use banking::savings_account::SavingsAccount;
use institution::bank::Bank;
use institution::credit_union::CreditUnion;
use institution::institution::Institution;
use institution::membership::Membership;

fn marcus_stuff() {
    let mut checking: CheckingAccount = CheckingAccount::new(500.0, 200.0);
    let mut savings: SavingsAccount = SavingsAccount::new(1000.0, 0.02);
    checking.deposit(150.0);
    savings.apply_interest();

    let mut bank = Bank::new("First Bank".to_string(), "4670 E BLVD".to_string());
    let mut credit_union = CreditUnion::new(
        "Dakotah Credit".to_string(),
        "67544 W BLVD".to_string(),
        Membership::Gold,
    );

    bank.add_account(Box::new(savings));
    bank.apply_annual_fee();
    credit_union.base.add_account(Box::new(checking));
    credit_union.apply_annual_fee();

    for account in bank.accounts() {
        println!("Account balance: {}", account.get_balance());
    }

    for account in credit_union.accounts() {
        println!("Account balance: {}", account.get_balance());
    }
}

fn main() {
    let mut checking: CheckingAccount = CheckingAccount::new(500.0, 200.0);
    let mut savings: SavingsAccount = SavingsAccount::new(1000.0, 0.02);

    println!("Checking ID: {}", checking.get_id());

    checking.deposit(150.0);
    savings.apply_interest();

    println!("Checking balance: {}", checking.get_balance());
    println!("Savings balance: {}", savings.get_balance());

    marcus_stuff();
}
