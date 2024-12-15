/*==========================================================================
 * Author: Cade Jacobson
 * Description: This file is where we define the trait for the Account so
 * that way we have it as an interface like fashion.
========================================================================== */

pub trait Account {
    fn get_balance(&self) -> f64;
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn take_loan(&mut self, loan_amount: f64);
    fn get_id(&self) -> String;
}
