/*==========================================================================
 * Author: Marcus Kane
 * Description: This file is where hte CreditUnion struct is defined, along with
 * its implementaion making it a class like style so we can use CreditUnion
 * in an OOP fashion. CreditUnion also inplements the trait Institution
========================================================================== */

use super::base_institution::BaseInstitution;
use super::institution::Institution;
use super::membership::Membership;
use crate::banking::account::Account;

pub struct CreditUnion {
    pub base: BaseInstitution,
    pub membership: Membership,
}

impl CreditUnion {
    /*==========================================================================
    * Author: Marcus Kane
    * Description: This is the initializer for CreditUnion
    * Parameter: name which is a string that represents the name of the
    * CreditUnion, address which is a string that represents the address of
    * the CreditUnion, membership which is of type membership that
    * represents the membership had in the  CreditUnion
    * Return: this function returns a new instance of the CreditUnion
    ========================================================================== */
    pub fn new(name: String, address: String, membership: Membership) -> Self {
        CreditUnion {
            base: BaseInstitution::new(name, address),
            membership: membership,
        }
    }

    /*==========================================================================
    * Author: Marcus Kane
    * Description: This function changes the CreditUnion membership
    * to whatever the passed in value is
    * Parameter: amount - membership which is of type membership
    * that represents the membership to change to
    * Return: None
    ========================================================================== */
    pub fn change_membership(&mut self, membership: Membership) {
        self.membership = membership;
    }
}

impl Institution for CreditUnion {
    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns the name of the Credit Union
     * Parameter: None
     * Return: &str string name of the Credit Union
    ========================================================================== */
    fn name(&self) -> &str {
        &self.base.name
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns the address of the Credit Union
     * Parameter: None
     * Return: &str string address of the Credit Union
    ========================================================================== */
    fn address(&self) -> &str {
        &self.base.address
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This function returns an array of all the Accounts in
     * the Credit Union
     * Parameter: None
     * Return: &Vec<Box<dyn Account>> array of all the Accounts in the
     * Credit Union
    ========================================================================== */
    fn accounts(&self) -> &Vec<Box<dyn Account>> {
        &self.base.accounts
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This funcion adds the passed in account to the Credit Union
     * Parameter: account which is a Box<dyn Account> and represents the accout
     * that is to be added to the Credit Union
     * Return: None
    ========================================================================== */
    fn add_account(&mut self, account: Box<dyn Account>) {
        self.base.accounts.push(account);
    }

    /*==========================================================================
     * Author: Marcus Kane
     * Description: This funcion removes the passed in account from the
     * Credit Union
     * Parameter: account which is a Box<dyn Account> and represents the account
     * that is to be removed from the Credit Union
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
     * managed by the Credit Union.
     * Parameter: None
     * Return: None
    ========================================================================== */
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
