use crate::banking::loans::loan::Loan;
use crate::banking::utils::format_dollar;

pub struct Mortgage {
    pub principal: f64,
    pub annual_interest_rate: f64,
    pub term_years: u32,
    pub property_value: f64,
    pub property_tax_rate: f64, // as a percentage of property value
}

impl Loan for Mortgage {
    fn calculate_minimum_monthly_payment(&self) -> f64 {
        let rate = self.annual_interest_rate / 12.0 / 100.0;
        let n = (self.term_years * 12) as f64;
        self.principal * rate / (1.0 - (1.0 + rate).powf(-n))
    }

    fn total_interest(&self) -> f64 {
        let total_payments =
            self.calculate_minimum_monthly_payment() * (self.term_years * 12) as f64;
        total_payments - self.principal
    }

    fn calculate_payoff_time(&self, monthly_payment: f64) -> u32 {
        let monthly_interest_rate = self.annual_interest_rate / 100.0 / 12.0;

        if monthly_payment <= self.principal * monthly_interest_rate {
            panic!("Monthly payment is too low to cover interest. Loan will never be paid off!");
        }

        let numerator =
            (monthly_payment / (monthly_payment - self.principal * monthly_interest_rate)).ln();
        let denominator = (1.0 + monthly_interest_rate).ln();

        let payoff_time = numerator / denominator;

        payoff_time.ceil() as u32
    }

    fn calculate_early_payment_savings(&self, num_months_early: u32) -> f64 {
        let monthly_rate = self.annual_interest_rate / 100.0 / 12.0;
        let total_months = self.term_years * 12;

        if num_months_early == 0 {
            panic!("Number of months early must be greater than zero.");
        }
        if num_months_early >= total_months {
            panic!("Cannot pay off the loan earlier than its total term.");
        }

        let remaining_months = total_months - num_months_early;

        let remaining_balance = self.principal
            * (1.0 - (1.0 + monthly_rate).powi(-(remaining_months as i32)))
            / (1.0 - (1.0 + monthly_rate).powi(-(total_months as i32)));

        let total_payment = self.principal
            * (monthly_rate * (1.0 + monthly_rate).powi(total_months as i32))
            / ((1.0 + monthly_rate).powi(total_months as i32) - 1.0);

        let total_interest_full_term = total_payment * total_months as f64 - self.principal;

        let interest_paid_early = total_payment * remaining_months as f64 - remaining_balance;

        total_interest_full_term - interest_paid_early
    }

    fn format_details(&self) -> String {
        format!(
            "Principal: {}\n\
       Interest Rate: {:.2}%\n\
       Term: {} years\n\
       Property Value: {}\n\
       Property Tax Rate: {:.2}%\n\
       Minimum Monthly Payment: {}\n\
       Total Interest: {}\n\
       Annual Property Tax: {}\n\
       ",
            format_dollar(self.principal),
            self.annual_interest_rate,
            self.term_years,
            format_dollar(self.property_value),
            self.property_tax_rate,
            format_dollar(self.calculate_minimum_monthly_payment()),
            format_dollar(self.total_interest()),
            format_dollar(self.calculate_property_tax()),
        )
    }
}

impl Mortgage {
    pub fn calculate_property_tax(&self) -> f64 {
        self.property_value * self.property_tax_rate / 100.0
    }
}
