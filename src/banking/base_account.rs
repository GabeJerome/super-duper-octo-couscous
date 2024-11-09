use super::account::Account;

pub struct BaseAccount {
    pub balance: f64,
    pub loan: f64,
}

impl BaseAccount {
    pub fn new(balance: f64) -> Self {
        BaseAccount { balance, loan: 0.0 }
    }
}

impl Account for BaseAccount {
    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    fn take_loan(&mut self, loan_amount: f64) {
        self.loan += loan_amount;
        self.balance += loan_amount;
    }
}
