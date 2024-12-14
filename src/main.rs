mod banking;
mod institution;
use std::io;

use crate::banking::account::Account;
use crate::banking::loans::loan::Loan;
use crate::banking::utils::format_dollar;
use banking::checking_account::CheckingAccount;
use banking::loans::auto_loan::AutoLoan;
use banking::loans::mortgage::Mortgage;
use banking::savings_account::SavingsAccount;
use institution::bank::Bank;
use institution::credit_union::CreditUnion;
use institution::institution::Institution;
use institution::membership::Membership;
/*==========================================================================
* Author: Marcus Kane
* Description: This function runs the example tests for how Institutions 
* could be used
* Parameter: None
* Return: None
========================================================================== */
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

    let account_to_remove = Box::new(SavingsAccount::new(1000.0, 0.02));
    let savings_account = Box::new(savings);
    bank.add_account(savings_account);
    bank.apply_annual_fee();
    bank.add_investment_funds(10.00);
    credit_union.base.add_account(Box::new(checking));
    credit_union.apply_annual_fee();
    credit_union.change_membership(Membership::Premium);

    wait_for_enter();
    println!("\n--- Apply Annual Fees ---");

    println!("Bank name: {}", bank.name());
    println!("Bank address: {}", bank.address());
    println!(
        "Bank ivestment funds: {}",
        format_dollar(bank.get_investment_funds())
    );

    for account in bank.accounts() {
        println!(
            "Bank account balance: {}",
            format_dollar(account.get_balance())
        );
    }
    bank.remove_account(account_to_remove);

    println!("");
    println!("Credit Union name: {}", credit_union.name());
    println!("Credit Union address: {}", credit_union.address());
    credit_union.change_membership(Membership::Standard);
    for account in credit_union.accounts() {
        println!(
            "Credit Union account balance: {}",
            format_dollar(account.get_balance())
        );
    }
}

fn cade_stuff() {
    println!("\n--- Creating & Adding Accounts ---");
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

    wait_for_enter();
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

    wait_for_enter();
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

fn wait_for_enter() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
}

fn main() {
    cade_stuff();
    marcus_stuff();
    gabe_stuff();
    print!("\n");
}
