use alloy::primitives::{Address, I256, U256};
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;
use tera::{Context, Tera};

use crate::github::bot::Bot;

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

pub fn close_issue(grant_amount: U256, developer_address: Address, github_id: I256) -> String {
    let developer_address_str = developer_address.to_string();
    let amount_str = format_type(grant_amount);
    let github_id = format_type(github_id);
    let github_name = "Hello";

    let context = HashMap::from([
        ("grant_amount", amount_str.as_str()),
        ("developer_address", developer_address_str.as_str()),
        ("github_id", github_id.as_str()),
        ("github_name", github_name),

    ]);

    generate_template(context, "close_issue.tera")
}

pub fn open_issue(grant_creator_address: Address, grant_amount: U256, deadline: U256) -> String {
    let grant_creator_address_str = grant_creator_address.to_string();
    let grant_amount_str = format_type(grant_amount);
    let deadline_str = u256_to_utc_string(deadline);

    let context = HashMap::from([
        ("grant_creator_address", grant_creator_address_str.as_str()),
        ("grant_amount", grant_amount_str.as_str()),
        ("deadline", deadline_str.as_str()),
    ]);

    generate_template(context, "open_issue.tera")
}

pub fn increase_deadline(deadline: U256, new_deadline: U256) -> String {
    let deadline = &u256_to_utc_string(deadline);
    let new_deadline = &u256_to_utc_string(new_deadline);

    let context = HashMap::from([
        ("deadline", deadline.as_str()),
        ("new_deadline", new_deadline.as_str()),
    ]);
    generate_template(context, "increase_deadline.tera")
}

#[tokio::test]
async fn test_close_issue() {
    let grant_amount = U256::from(1_000_000_000_000_000_000u128); // 1 ETH in wei
    let developer_address: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"
        .parse()
        .unwrap();
    let github_id = I256::ZERO();

    let bot = Bot::try_new("").unwrap();
    let _ = bot
        .add_issue_comment(
            "b3ww",
            "vGrant",
            9,
            &close_issue(
                grant_amount,
                developer_address,
                github_id
            ),
        )
        .await;
    ()
}
