mod banking;

use banking::checking_account::CheckingAccount;
use banking::savings_account::SavingsAccount;
use crate::banking::account::Account;

fn main() {
    let mut checking = CheckingAccount::new(500.0, 200.0);
    let mut savings = SavingsAccount::new(1000.0, 0.02);

    println!("Checking ID: {}", checking.get_id());

    checking.deposit(150.0);
    savings.apply_interest();

    println!("Checking balance: {}", checking.get_balance());
    println!("Savings balance: {}", savings.get_balance());
}
