// Author: Gabe Jerome
// Description: This file contains utility functions for the bankning module

use num_format::{Locale, ToFormattedString};

// Description: Converts a float 64 into a string in standard U.S. dollar format
// Parameter: amount - the float 64 value to be represented as a dollar amount
// Return: The formatted amount as a string in standard U.S. dollar format
pub fn format_dollar(amount: f64) -> String {
    let dollars = amount.trunc() as i64;
    let cents = (amount.fract() * 100.0).round() as u64;
    format!("${}.{:02}", dollars.to_formatted_string(&Locale::en), cents)
}
