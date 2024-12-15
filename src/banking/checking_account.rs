use super::account::Account;
use super::base_account::BaseAccount;

/*==========================================================================
* Author: Cade Jacobson
* Description: This file defines the CheckingAccount stuct 
========================================================================== */
pub struct CheckingAccount {
    pub base: BaseAccount,
    pub overdraft_limit: f64,
}

impl CheckingAccount {
    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This is the initializer for the checking account
    * Parameter: balance - a double indicating the amount stored in the account.
    * Parameter: overdraft_limit - a double indicating maximum overdraft value
    * Return: this function returns a new instance of the CheckingAccount
    ========================================================================== */
    pub fn new(balance: f64, overdraft_limit: f64) -> Self {
        CheckingAccount {
            base: BaseAccount::new(balance),
            overdraft_limit,
        }
    }
}

impl Account for CheckingAccount {
    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function returns the account ID.
    * Parameter: None
    * Return: String which represents the account's ID
    ========================================================================== */
    fn get_id(&self) -> String {
        self.base.id.to_string()
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function returns the account balance.
    * Parameter: None
    * Return: 64 bit floating number which represents the account's balance
    ========================================================================== */
    fn get_balance(&self) -> f64 {
        self.base.get_balance()
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function adds an amount to the balance an account holds.
    * Parameter: amount - a 64 bit floating point value to deposit to an account
    * Return: None
    ========================================================================== */
    fn deposit(&mut self, amount: f64) {
        self.base.deposit(amount);
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function withdraws a certain amount of money from an account
    * Parameter: amount - a 64 bit floating point value to remove from an account
    * Return: Result<> data type indicating success or failure.
    ========================================================================== */
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.get_balance() + self.overdraft_limit >= amount {
            self.base.balance -= amount;
            Ok(())
        } else {
            Err("Overdraft limit exceeded".to_string())
        }
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function applies a loan to a given account.
    * Parameter: loan_amount - the value of the loan being applied to an account.
    * Return: None
    ========================================================================== */
    fn take_loan(&mut self, loan_amount: f64) {
        self.base.take_loan(loan_amount);
    }
}
