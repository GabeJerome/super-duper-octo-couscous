use super::account::Account;
use uuid::Uuid;


pub struct BaseAccount {
    pub id: Uuid, 
    pub balance: f64,
    pub loan: f64,
}

impl BaseAccount {
    pub fn new(balance: f64) -> Self {
        let id = Uuid::new_v4();
        BaseAccount { id, balance, loan: 0.0 }
    }
}

impl Account for BaseAccount {
    fn get_id(&self) -> String {
        self.id.to_string()
    }

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
