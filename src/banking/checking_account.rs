use super::account::Account;
use super::base_account::BaseAccount;

pub struct CheckingAccount {
    pub base: BaseAccount,
    pub overdraft_limit: f64,
}

impl CheckingAccount {
    pub fn new(balance: f64, overdraft_limit: f64) -> Self {
        CheckingAccount {
            base: BaseAccount::new(balance),
            overdraft_limit,
        }
    }
}

impl Account for CheckingAccount {
    fn get_id(&self) -> String {
        self.base.id.to_string()
    }

    fn get_balance(&self) -> f64 {
        self.base.get_balance()
    }

    fn deposit(&mut self, amount: f64) {
        self.base.deposit(amount);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.get_balance() + self.overdraft_limit >= amount {
            self.base.balance -= amount;
            Ok(())
        } else {
            Err("Overdraft limit exceeded".to_string())
        }
    }

    fn take_loan(&mut self, loan_amount: f64) {
        self.base.take_loan(loan_amount);
    }
}
