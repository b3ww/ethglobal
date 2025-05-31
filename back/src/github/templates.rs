use alloy::primitives::{Address, I256, U256};
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use tera::{Context, Tera};

fn generate_template(values: HashMap<&str, &str>, name: &str) -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Erreur lors du chargement des templates");
    let context = values.iter().fold(Context::new(), |mut ctx, (key, value)| {
        ctx.insert(*key, value);
        ctx
    });

    tera.render(name, &context).unwrap()
}

fn format_type<T>(elem: T) -> String
where
    T: std::fmt::Display,
{
    elem.to_string()
}

pub fn u256_to_utc_string(timestamp: U256) -> String {
    let seconds = timestamp.to::<u64>();
    let datetime: DateTime<Utc> = Utc.timestamp_opt(seconds as i64, 0).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

#[allow(dead_code)]
pub fn close_issue(
    grant_amount: U256,
    developer_name: String,
    developer_address: Address,
    tx_hash: Address,
) -> String {
    let contract_address_str = contract_address.to_string();
    let developer_address_str = developer_address.to_string();
    let tx_hash_str = tx_hash.to_string();
    let amount_str = format_type(grant_amount);

    let context = HashMap::from([
        ("contract_address", contract_address_str.as_str()),
        ("grant_amount", amount_str.as_str()),
        ("developer_name", developer_name.as_str()),
        ("developer_address", developer_address_str.as_str()),
        ("tx_hash", tx_hash_str.as_str()),
    ]);

    generate_template(context, "close_issue.tera")
}

pub fn open_issue(
    grant_creator_address: Address,
    grant_amount: U256,
    deadline: U256,
) -> String {
    let grant_creator_address_str = grant_creator_address.to_string();
    let contract_address_str = contract_address.to_string();
    let grant_amount_str = format_type(grant_amount);
    let deadline_str = u256_to_utc_string(deadline);

    let context = HashMap::from([
        ("grant_creator_address", grant_creator_address_str.as_str()),
        ("grant_amount", grant_amount_str.as_str()),
        ("contract_address", contract_address_str.as_str()),
        ("deadline", deadline_str.as_str()),
    ]);

    generate_template(context, "open_issue.tera")
}

pub fn increase_deadline(
    deadline: U256,
    new_deadline: U256,
    github_id: I256,
) -> String {
    let deadline = &u256_to_utc_string(deadline);
    let new_deadline = &u256_to_utc_string(new_deadline);
    let contract_address = &contract_address.to_string();

    let context = HashMap::from([
        ("contract_address", contract_address.as_str()),
        ("deadline",     deadline.as_str()),
        ("new_deadline", new_deadline.as_str()),
    ]);
    generate_template(context, "increase_deadline.tera")
}
