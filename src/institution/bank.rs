use super::base_institution::BaseInstitution;
use super::institution::Institution;
use crate::banking::account::Account;

pub struct Bank {
    pub base: BaseInstitution,
    pub investment_funds: f64,
}

impl Bank {
    pub fn new(name: String, address: String) -> Self {
        Bank {
            base: BaseInstitution::new(name, address),
            investment_funds: 0.0,
        }
    }

    pub fn add_investment_funds(&mut self, amount: f64) {
        self.investment_funds += amount;
        println!("Investment funds updated: {}", self.investment_funds);
    }

    pub fn get_investment_funds(&self) -> f64 {
        self.investment_funds
    }
}

impl Institution for Bank {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn address(&self) -> &str {
        &self.base.address
    }

    fn accounts(&self) -> &Vec<Box<dyn Account>> {
        &self.base.accounts
    }

    fn add_account(&mut self, account: Box<dyn Account>) {
        self.base.accounts.push(account);
    }

    fn remove_account(&mut self, account: Box<dyn Account>) {
        if let Some(pos) = self
            .base
            .accounts
            .iter()
            .position(|a| a.get_id() == account.get_id())
        {
            self.base.accounts.remove(pos);
        }
    }

    fn apply_annual_fee(&mut self) {
        let fee = 10.00;
        for account in &mut self.base.accounts {
            if account.get_balance() >= fee {
                account.withdraw(fee).ok();
            } else {
                account.withdraw(account.get_balance()).ok();
            }
        }
    }
}
