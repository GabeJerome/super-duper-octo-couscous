use num_format::{Locale, ToFormattedString};

pub fn format_dollar(amount: f64) -> String {
    let dollars = amount.trunc() as i64;
    let cents = (amount.fract() * 100.0).round() as u64;
    format!(
        "${}.{:02}",
        dollars.to_formatted_string(&Locale::en),
        cents
    )
}