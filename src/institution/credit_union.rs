use super::base_institution::BaseInstitution;
use super::institution::Institution;
use super::membership::Membership;
use crate::banking::account::Account;

pub struct CreditUnion {
    pub base: BaseInstitution,
    pub membership: Membership,
}

impl CreditUnion {
    pub fn new(name: String, address: String, membership: Membership) -> Self {
        CreditUnion {
            base: BaseInstitution::new(name, address),
            membership: membership,
        }
    }

    pub fn change_membership(&mut self, membership: Membership) {
        self.membership = membership;
    }
}

impl Institution for CreditUnion {
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
        let fee = self.membership.fee();
        for account in &mut self.base.accounts {
            if account.get_balance() >= fee {
                account.withdraw(fee).ok();
            } else {
                account.withdraw(account.get_balance()).ok();
            }
        }
    }
}
