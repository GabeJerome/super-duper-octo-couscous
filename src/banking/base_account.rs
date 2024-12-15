use super::account::Account;
use uuid::Uuid;


/*==========================================================================
* Author: Cade Jacobson
* Description: This file defines the BaseAccount stuct that checking and
* savings accounts use as a base
========================================================================== */
pub struct BaseAccount {
    pub id: Uuid, 
    pub balance: f64,
    pub loan: f64,
}

impl BaseAccount {

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This is the initializer for the base account
    * Parameter: balance - a double indicating the amount stored in the account.
    * Return: this function returns a new instance of the BaseAccount
    ========================================================================== */
    pub fn new(balance: f64) -> Self {
        let id = Uuid::new_v4();
        BaseAccount { id, balance, loan: 0.0 }
    }
}

impl Account for BaseAccount {
    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function returns the account ID.
    * Parameter: None
    * Return: String which represents the account's ID
    ========================================================================== */
    fn get_id(&self) -> String {
        self.id.to_string()
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function returns the account balance.
    * Parameter: None
    * Return: 64 bit floating number which represents the account's balance
    ========================================================================== */
    fn get_balance(&self) -> f64 {
        self.balance
    }

    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function adds an amount to the balance an account holds.
    * Parameter: amount - a 64 bit floating point value to deposit to an account
    * Return: None
    ========================================================================== */
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }


    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function withdraws a certain amount of money from an account
    * Parameter: amount - a 64 bit floating point value to remove from an account
    * Return: Result<> data type indicating success or failure.
    ========================================================================== */
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }


    /*==========================================================================
    * Author: Cade Jacobson
    * Description: This function applies a loan to a given account.
    * Parameter: loan_amount - the value of the loan being applied to an account.
    * Return: None
    ========================================================================== */
    fn take_loan(&mut self, loan_amount: f64) {
        self.loan += loan_amount;
        self.balance += loan_amount;
    }
}
