mod banking;
mod institution;

use crate::banking::account::Account;
use banking::checking_account::CheckingAccount;
use banking::savings_account::SavingsAccount;
use institution::bank::Bank;
use institution::credit_union::CreditUnion;
use institution::institution::Institution;
use institution::membership::Membership;
use crate::banking::utils::format_dollar;
use crate::banking::loans::loan::Loan;
use banking::loans::auto_loan::AutoLoan;
use banking::loans::mortgage::Mortgage;

fn marcus_stuff() {
    let mut checking: CheckingAccount = CheckingAccount::new(500.0, 200.0);
    let mut savings: SavingsAccount = SavingsAccount::new(1000.0, 0.02);
    checking.deposit(150.0);
    savings.apply_interest();

    let mut bank = Bank::new("First Bank".to_string(), "4670 E BLVD".to_string());
    let mut credit_union = CreditUnion::new(
        "Dakotah Credit".to_string(),
        "67544 W BLVD".to_string(),
        Membership::Gold,
    );

    bank.add_account(Box::new(savings));
    bank.apply_annual_fee();
    credit_union.base.add_account(Box::new(checking));
    credit_union.apply_annual_fee();

    for account in bank.accounts() {
        println!("Account balance: {}", account.get_balance());
    }

    for account in credit_union.accounts() {
        println!("Account balance: {}", account.get_balance());
    }
}

fn cade_stuff() {
    let mut checking = CheckingAccount::new(500.0, 200.0);
    let mut savings = SavingsAccount::new(1000.0, 0.02);

    println!("Checking ID: {}", checking.get_id());

    checking.deposit(150.0);
    savings.apply_interest();
  
    println!(
        "Checking balance: {}",
        format_dollar(checking.get_balance())
    );
    println!("Savings balance: {}", format_dollar(savings.get_balance()));
}

fn gabe_stuff() {
    let mortgage = Mortgage {
        principal: 300_000.0,
        annual_interest_rate: 3.5,
        term_years: 30,
        property_value: 400_000.0,
        property_tax_rate: 1.25,
    };

    let auto_loan = AutoLoan {
        principal: 30_000.0,
        annual_interest_rate: 5.0,
        term_years: 5,
        trade_in_value: 5_000.0,
        car_value: 25_000.0,
        annual_depreciation_rate: 15.0,
    };

    println!("\n--- Mortgage ---");
    println!("{}", mortgage.format_details());

    // Probably get this from the user in the future maybe?
    let custom_mortgage_payment = 1400.0;
    let num_months = mortgage.calculate_payoff_time(custom_mortgage_payment);
    let num_months_early = (mortgage.term_years * 12) - num_months;
    println!(
        "Time to pay off with {}/month: {} months",
        format_dollar(custom_mortgage_payment),
        num_months
    );
    println!(
        "Early Payment Savings ({} months early): {}",
        num_months_early,
        format_dollar(mortgage.calculate_early_payment_savings(num_months_early))
    );

    println!("\n--- Auto Loan ---");
    println!("{}", auto_loan.format_details());

    // Probably get this from the user in the future maybe?
    let custom_auto_payment = 500.0;
    let num_months = auto_loan.calculate_payoff_time(custom_auto_payment);
    let num_months_early = (auto_loan.term_years * 12) - num_months;
    println!(
        "Time to pay off with {}/month: {} months",
        format_dollar(custom_auto_payment),
        num_months
    );
    println!(
        "Early Payment Savings ({} months early): {}",
        num_months_early,
        format_dollar(auto_loan.calculate_early_payment_savings(num_months_early))
    );
}

fn main() {
    cade_stuff();
    marcus_stuff();
    gabe_stuff();
}
