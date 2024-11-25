use crate::banking::account::Account;

pub trait Institution {
    fn name(&self) -> &str;
    fn address(&self) -> &str;
    fn accounts(&self) -> &Vec<Box<dyn Account>>;
    fn add_account(&mut self, account: Box<dyn Account>);
    fn remove_account(&mut self, account: Box<dyn Account>);
    fn apply_annual_fee(&mut self);
}
