use crate::banking::loans::loan::Loan;
use crate::banking::utils::format_dollar;

const BASE_INSURANCE_RATE: f64 = 0.05;

pub struct AutoLoan {
  pub principal: f64,
  pub annual_interest_rate: f64,
  pub term_years: u32,
  pub trade_in_value: f64,
  pub car_value: f64,
  pub annual_depreciation_rate: f64,
}

impl Loan for AutoLoan {
  fn calculate_minimum_monthly_payment(&self) -> f64 {
      let adjusted_principal = self.principal - self.trade_in_value;
      let rate = self.annual_interest_rate / 12.0 / 100.0;
      let n = (self.term_years * 12) as f64;
      adjusted_principal * rate / (1.0 - (1.0 + rate).powf(-n))
  }

  fn total_interest(&self) -> f64 {
      let total_payments = self.calculate_minimum_monthly_payment() * (self.term_years * 12) as f64;
      total_payments - (self.principal - self.trade_in_value)
  }
  
  fn calculate_payoff_time(&self, monthly_payment: f64) -> u32 {
    let adjusted_principal = self.principal - self.trade_in_value;
    let monthly_interest_rate = self.annual_interest_rate / 100.0 / 12.0;

    if monthly_payment <= adjusted_principal * monthly_interest_rate {
        panic!("Monthly payment is too low to cover interest. Loan will never be paid off!");
    }

    let numerator = (monthly_payment / (monthly_payment - adjusted_principal * monthly_interest_rate)).ln();
    let denominator = (1.0 + monthly_interest_rate).ln();

    let payoff_time = numerator / denominator;

    payoff_time.ceil() as u32
  }
  
  fn calculate_early_payment_savings(&self, num_months_early: u32) -> f64 {
    let adjusted_principal = self.principal - self.trade_in_value;
    let monthly_rate = self.annual_interest_rate / 100.0 / 12.0;
    let total_months = self.term_years * 12;

    if num_months_early == 0 {
        panic!("Number of months early must be greater than zero.");
    }
    if num_months_early >= total_months {
        panic!("Cannot pay off the loan earlier than its total term.");
    }

    let remaining_months = total_months - num_months_early;

    let remaining_balance = adjusted_principal
        * (1.0 - (1.0 + monthly_rate).powi(-(remaining_months as i32)))
        / (1.0 - (1.0 + monthly_rate).powi(-(total_months as i32)));

    let total_payment = adjusted_principal
        * (monthly_rate * (1.0 + monthly_rate).powi(total_months as i32))
        / ((1.0 + monthly_rate).powi(total_months as i32) - 1.0);

    let total_interest_full_term = total_payment * total_months as f64 - adjusted_principal;

    let interest_paid_early = total_payment * remaining_months as f64 - remaining_balance;

    total_interest_full_term - interest_paid_early
    }
  
  fn format_details(&self) -> String {
    format!(
      "Principal: {}\n\
       Interest Rate: {:.2}%\n\
       Term: {} years\n\
       Trade-In Value: {}\n\
       Car Value: {}\n\
       Annual Depreciation Rate: {:.2}%\n\
       Minimum Monthly Payment: {}\n\
       Total Interest: {}\n\
       Car Value After Depreciation: {}\n\
       Monthly Insurance Cost: {}\n",
      format_dollar(self.principal),
      self.annual_interest_rate,
      self.term_years,
      format_dollar(self.trade_in_value),
      format_dollar(self.car_value),
      self.annual_depreciation_rate,
      format_dollar(self.calculate_minimum_monthly_payment()),
      format_dollar(self.total_interest()),
      format_dollar(self.calculate_depreciation()),
      format_dollar(self.calculate_insurance_cost())
  )
    }
}

impl AutoLoan {
  pub fn calculate_depreciation(&self) -> f64 {
      let years = self.term_years as f64;
      self.car_value * (1.0 - self.annual_depreciation_rate / 100.0).powf(years)
  }

  pub fn calculate_insurance_cost(&self) -> f64 {
      self.car_value * BASE_INSURANCE_RATE / 12.0
  }
}
