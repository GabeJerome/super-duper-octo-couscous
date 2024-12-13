/*==========================================================================
 * Author: Marcus Kane
 * Description: This file is where hte bank struct is defined, along with
 * its implementaion making it a class like style so we can use bank
 * in an OOP fashion. Bank also inplements the trait Institution
========================================================================== */

use super::base_institution::BaseInstitution;
use super::institution::Institution;
use crate::banking::account::Account;
use crate::banking::utils::format_dollar;

pub struct Bank {
    pub base: BaseInstitution,
    pub investment_funds: f64,
}

impl Bank {
    /*==========================================================================
    * Author: Marcus Kane
    * Description: This is the initializer for Bank
    * Parameter: name which is a string that represents the name of the
    * Bank, address which is a string that represents the address of
    * the bank
    * Return: this function returns a new instance of the Bank
    ========================================================================== */
    pub fn new(name: String, address: String) -> Self {
        Bank {
            base: BaseInstitution::new(name, address),
            investment_funds: 0.0,
        }
    }

    /*==========================================================================
    * Author: Marcus Kane
    * Description: This function adds the specified amount to the banks
    * investment fundsand then prints the updated total.
    * Parameter: amount - A positive or negative f64 value which represents
    * the amount to add to the investment funds.
    * Return: None
    ========================================================================== */
    pub fn add_investment_funds(&mut self, amount: f64) {
        self.investment_funds += amount;
        println!(
            "Investment funds updated: {}",
            format_dollar(self.investment_funds)
        );
    }

    /*==========================================================================
    * Author: Marcus Kane
    * Description: This function returns the value of the investment funds
    * Parameter: None
    * Return: positive or negative f64 value of the investment funds
    ========================================================================== */
    pub fn get_investment_funds(&self) -> f64 {
        self.investment_funds
    }
}

impl Institution for Bank {
    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns the name of the Bank
     * Parameter: None
     * Return: &str string name of the Bank
    ========================================================================== */
    fn name(&self) -> &str {
        &self.base.name
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns the address of the Bank
     * Parameter: None
     * Return: &str string address of the Bank
    ========================================================================== */
    fn address(&self) -> &str {
        &self.base.address
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns an array of all the Accounts in
     * the Bank
     * Parameter: None
     * Return: &Vec<Box<dyn Account>> array of all the Accounts in the Bank
    ========================================================================== */
    fn accounts(&self) -> &Vec<Box<dyn Account>> {
        &self.base.accounts
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This funcion adds the passed in account to the Bank
     * Parameter: account which is a Box<dyn Account> and represents the accout
     * that is to be added to the Bank
     * Return: None
    ========================================================================== */
    fn add_account(&mut self, account: Box<dyn Account>) {
        self.base.accounts.push(account);
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This funcion removes the passed in account from the Bank
     * Parameter: account which is a Box<dyn Account> and represents the account
     * that is to be removed from the Bank
     * Return: None
    ========================================================================== */
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

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function applies the annual fee to all the accounts
     *  managed by the bank.
     * Parameter: None
     * Return: None
    ========================================================================== */
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
