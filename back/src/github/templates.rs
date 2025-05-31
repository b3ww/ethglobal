use alloy::{dyn_abi::abi, primitives::Address, primitives::U256};
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::collections::HashMap;
use tera::{Context, Tera};
use tracing_subscriber::fmt::time::SystemTime;

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

pub fn close_issue(
    contract_address: Address,
    grant_amount: U256,
    developer_name: String,
    developer_address: Address,
    gas_fee: U256,
    tx_hash: Address,
) -> String {
    let contract_address_str = contract_address.to_string();
    let developer_address_str = developer_address.to_string();
    let tx_hash_str = tx_hash.to_string();
    let amount_str = format_type(grant_amount);
    let gas_fee_str = format_type(gas_fee);

    let context = HashMap::from([
        ("contract_address", contract_address_str.as_str()),
        ("grant_amount", amount_str.as_str()),
        ("developer_name", developer_name.as_str()),
        ("developer_address", developer_address_str.as_str()),
        ("gas_fee", gas_fee_str.as_str()),
        ("tx_hash", tx_hash_str.as_str()),
    ]);

    generate_template(context, "close_issue.tera")
}

pub fn open_issue(
    grant_creator_address: Address,
    grant_amount: U256,
    contract_address: Address,
    gas_fee: U256,
    tx_hash: Address,
    deadline: U256,
) -> String {
    let grant_creator_address_str = grant_creator_address.to_string();
    let contract_address_str = contract_address.to_string();
    let tx_hash_str = tx_hash.to_string();
    let grant_amount_str = format_type(grant_amount);
    let gas_fee_str = format_type(gas_fee);
    let deadline_str = u256_to_utc_string(deadline);

    let context = HashMap::from([
        ("grant_creator_address", grant_creator_address_str.as_str()),
        ("grant_amount", grant_amount_str.as_str()),
        ("contract_address", contract_address_str.as_str()),
        ("gas_fee", gas_fee_str.as_str()),
        ("tx_hash", tx_hash_str.as_str()),
        ("deadline", deadline_str.as_str()),
    ]);

    generate_template(context, "open_issue.tera")
}

pub fn increase_deadline(
    gas_fee: U256,
    tx_hash: Address,
    deadline: U256,
    new_deadline: U256,
    contract_address: Address,
) -> String {
    let gas_fee = &format_type(gas_fee);
    let deadline = &u256_to_utc_string(deadline);
    let new_deadline = &u256_to_utc_string(new_deadline);
    let contract_address = &contract_address.to_string();
    let tx_hash = &tx_hash.to_string();

    let context = HashMap::from([
        ("tx_hash", tx_hash.as_str()),
        ("contract_address", contract_address.as_str()),
        ("gas_fee", &gas_fee.as_str()),
        ("deadline", deadline.as_str()),
        ("new_deadline", new_deadline.as_str()),
    ]);
    generate_template(context, "increase_deadline.tera")
}

// Dead code
pub fn new_pull_request() -> String {
    let context = HashMap::from([
        ("developer_name", "vitalik"),
        ("developer_address", "0xABC123...DEF"),
        ("gas_fee", "0.1"),
        ("tx_hash", "0x12345...XYZ"),
    ]);

    generate_template(context, "new_pull_request.tera")
}

mod test {
    use alloy::primitives::{Address, U256};

    use crate::github::{
        Bot,
        templates::{increase_deadline, open_issue},
    };

    #[tokio::test]
    async fn toto() {
        let grant_creator_address: Address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
            .parse()
            .unwrap();

        let grant_amount = U256::from(1_000_000_000_000_000_000u128); // 1 ETH in wei
        let contract_address: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"
            .parse()
            .unwrap();

        let gas_fee = U256::from(100_000_000_000u128); // 100 Gwei
        let tx_hash: Address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
            .parse()
            .unwrap();

        // Deadline: current timestamp + 30 days
        let deadline = U256::from(chrono::Utc::now().timestamp() as u64 + 30 * 24 * 60 * 60);

        let bot = Bot::try_new("").unwrap();
        let _ = bot
            .add_issue_comment(
                "b3ww",
                "vGrant",
                9,
                &open_issue(
                    grant_creator_address,
                    grant_amount,
                    contract_address,
                    gas_fee,
                    tx_hash,
                    deadline,
                ),
            )
            .await;
        ()
    }
}
