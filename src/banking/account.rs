pub trait Account {
    fn get_balance(&self) -> f64;
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn take_loan(&mut self, loan_amount: f64);
}
