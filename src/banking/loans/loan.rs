pub trait Loan {
    fn calculate_minimum_monthly_payment(&self) -> f64;
    fn total_interest(&self) -> f64;
    fn calculate_payoff_time(&self, monthly_payment: f64) -> u32;
    fn calculate_early_payment_savings(&self, num_months_early: u32) -> f64;
    fn format_details(&self) -> String;
}
