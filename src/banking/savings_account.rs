use super::account::Account;
use super::base_account::BaseAccount;

pub struct SavingsAccount {
    pub base: BaseAccount,
    pub interest_rate: f64,
}

impl SavingsAccount {
    pub fn new(balance: f64, interest_rate: f64) -> Self {
        SavingsAccount {
            base: BaseAccount::new(balance),
            interest_rate,
        }
    }

    pub fn apply_interest(&mut self) {
        let interest = self.base.get_balance() * self.interest_rate;
        self.base.deposit(interest);
    }
}

impl Account for SavingsAccount {
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
        self.base.withdraw(amount)
    }

    fn take_loan(&mut self, loan_amount: f64) {
        self.base.take_loan(loan_amount);
    }
}
