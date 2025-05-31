use std::collections::HashMap;
use alloy::primitives::U256;
use chrono::{DateTime, Duration, TimeZone, Utc};
use tera::{Context, Tera};
use tracing_subscriber::fmt::time::SystemTime;

fn generate_template(values: HashMap<String, String>, name: &str) -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Erreur lors du chargement des templates");
    let context = values.iter().fold(Context::new(), |mut ctx, (key, value)| {
        ctx.insert(key, value);
        ctx
    });

    tera.render(name, &context).unwrap()
}

fn format_type<T>(elem: T) -> String
where
    T:std::fmt::Display
{
    elem.to_string()
}

pub fn u256_to_utc_string(timestamp: U256) -> String {
    let seconds = timestamp.to::<u64>();
    
    // Convert to DateTime<Utc>
    let datetime: DateTime<Utc> = Utc.timestamp_opt(seconds as i64, 0).unwrap();
    
    // Format as string (customize format as needed)
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn close_issue() -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Failed to load templates");

    let mut context = Context::new();
    context.insert("contract_address", "0xCONTRACT123...XYZ");
    context.insert("grant_amount", "100.00");
    context.insert("developer_name", "vitalik");
    context.insert("developer_address", "0xABC123...DEF");
    context.insert("gas_fee", "0.1");
    context.insert("tx_hash", "0x12345...XYZ");

    tera.render("close_issue.tera", &context).unwrap()
}

pub fn new_pull_request() -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Failed to load templates");

    let mut context = Context::new();
    context.insert("developer_name", "vitalik");
    context.insert("developer_address", "0xABC123...DEF");
    context.insert("gas_fee", "0.1");
    context.insert("tx_hash", "0x12345...XYZ");

    tera.render("new_pull_request.tera", &context).unwrap()
}


pub fn open_issue(amount: U256, deadline: U256) -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Failed to load templates");

    let mut context = Context::new();
    context.insert("grant_creator_address", "0xABC123...DEF");
    context.insert("grant_amount", &format_type(amount));
    context.insert("contract_address", "0xCONTRACT123...XYZ");
    context.insert("gas_fee", "0.1");
    context.insert("tx_hash", "0x12345...XYZ");
    context.insert("deadline", &u256_to_utc_string(deadline));

    tera.render("open_issue.tera", &context).unwrap()
}

pub fn increase_deadline() -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Failed to load templates");

    let mut context = Context::new();
    context.insert("gas_fee", "0.1");
    context.insert("tx_hash", "0x12345...XYZ");
    context.insert("deadline", "15 january");
    context.insert("new_deadline", "20 january");
    context.insert("contract_address", "0xCONTRACT123...XYZ");

    tera.render("increase_deadline.tera", &context).unwrap()
}


mod test {
    use crate::github::{templates::{increase_deadline, open_issue}, Bot};

    use super::close_issue;

    #[tokio::test]
    async fn toto() {
        let bot = Bot::try_new("").unwrap();
        let _ = bot.add_issue_comment("b3ww", "vGrant", 9, &increase_deadline()).await;
        ()
    }
}
