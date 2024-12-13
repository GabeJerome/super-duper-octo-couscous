/*==========================================================================
* Author: Marcus Kane
* Description: This file defines the BaseInstitution stuct that Banks and
* Credit Unions use as a base
========================================================================== */
use super::institution::Institution;
use crate::banking::account::Account;

pub struct BaseInstitution {
    pub name: String,
    pub address: String,
    pub accounts: Vec<Box<dyn Account>>,
}

impl BaseInstitution {
    /*==========================================================================
    * Author: Marcus Kane
    * Description: This is the initializer for base institution
    * Parameter: namae which is a string that represents the name of the
    * Institution, address which is a string that represents the address of
    * the institution
    * Return: this function returns a new instance of the BaseInstuitution
    ========================================================================== */
    pub fn new(name: String, address: String) -> Self {
        BaseInstitution {
            name,
            address,
            accounts: Vec::new(),
        }
    }
}

impl Institution for BaseInstitution {
    /*==========================================================================
    * Author: Marcus Kane
    * Description: This function returns the institution name.
    * Parameter: None
    * Return: &str which represents institutions name
    ========================================================================== */
    fn name(&self) -> &str {
        &self.name
    }

    /*==========================================================================
    * Author: Marcus Kane
    * Description: This function returns the institution address.
    * Parameter: None
    * Return: &str which represents institutions address
    ========================================================================== */
    fn address(&self) -> &str {
        &self.address
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns an array of all the Accounts in
     * the institution
     * Parameter: None
     * Return: &Vec<Box<dyn Account>> array of all the Accounts in the
     * institution
    ========================================================================== */
    fn accounts(&self) -> &Vec<Box<dyn Account>> {
        &self.accounts
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This funcion adds the passed in account to the institution
     * Parameter: account which is a Box<dyn Account> and represents the accout
     * that is to be added to the institution
     * Return: None
    ========================================================================== */
    fn add_account(&mut self, account: Box<dyn Account>) {
        self.accounts.push(account);
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This funcion removes the passed in account from the institution
     * Parameter: account which is a Box<dyn Account> and represents the account
     * that is to be removed from the institution
     * Return: None
    ========================================================================== */
    fn remove_account(&mut self, account: Box<dyn Account>) {
        if let Some(pos) = self
            .accounts
            .iter()
            .position(|a| a.get_id() == account.get_id())
        {
            self.accounts.remove(pos);
        }
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function applies the annual fee to all the accounts
     *  managed by the institution.
     * Parameter: None
     * Return: None
    ========================================================================== */
    fn apply_annual_fee(&mut self) {
        let fee = 10.00;
        for account in &mut self.accounts {
            if account.get_balance() >= fee {
                account.withdraw(fee).ok();
            } else {
                account.withdraw(account.get_balance()).ok();
            }
        }
    }
}
