use super::account::Account;
use super::base_account::BaseAccount;

/*==========================================================================
* Author: Cade Jacobson
* Description: This file defines the SavingsAccount stuct 
========================================================================== */
pub struct SavingsAccount {
    pub base: BaseAccount,
    pub interest_rate: f64,
}

impl SavingsAccount {
    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This is the initializer for the checking account
    * Parameter: balance - a double indicating the amount stored in the account.
    * Parameter: interest_rate - a double indicating the savings rate for the account.
    * Return: this function returns a new instance of the SavingsAccount
    ========================================================================== */
    pub fn new(balance: f64, interest_rate: f64) -> Self {
        SavingsAccount {
            base: BaseAccount::new(balance),
            interest_rate,
        }
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function applies an interest to a given account.
    * Parameter: None
    * Return: None
    ========================================================================== */
    pub fn apply_interest(&mut self) {
        let interest = self.base.get_balance() * self.interest_rate;
        self.base.deposit(interest);
    }
}

impl Account for SavingsAccount {
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
        self.base.withdraw(amount)
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
