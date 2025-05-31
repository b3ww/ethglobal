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

// fn m() -> String {
//     generate_template(HashMap::from([
//         ("grant_creator_address".to_string(), "0xABC123...DEF".to_string()),
//         ("grant_amount".to_string(), "100.00".to_string()),
//         ("contract_address".to_string(), "0xCONTRACT123...XYZ".to_string()),
//     ]), "open_issue.tera")
// }

pub fn close_issue() -> String {
    let context = HashMap::from([
        ("contract_address", "0xCONTRACT123...XYZ"),
        ("grant_amount", "100.00"),
        ("developer_name", "vitalik"),
        ("developer_address", "0xABC123...DEF"),
        ("gas_fee", "0.1"),
        ("tx_hash", "0x12345...XYZ"),
    ]);
    generate_template(context, "close_issue.tera")
}

pub fn new_pull_request() -> String {
    let context = HashMap::from([
        ("developer_name", "vitalik"),
        ("developer_address", "0xABC123...DEF"),
        ("gas_fee", "0.1"),
        ("tx_hash", "0x12345...XYZ"),
    ]);

    generate_template(context,"new_pull_request.tera")
}

pub fn open_issue() -> String {
    let context = HashMap::from([
        ("grant_creator_address", "0xCREATOR123...XYZ"),
        ("grant_amount", "100.00"),
        ("contract_address", "0xCONTRACT123...XYZ"),
        ("gas_fee", "0.1"),
        ("tx_hash", "0x12345...XYZ"),
    ]);

    generate_template(context, "open_issue.tera")
}

mod test {
    use crate::github::Bot;

    use super::close_issue;
    use super::open_issue;


    #[tokio::test]
    async fn toto() {
        let bot = Bot::try_new("").unwrap();
        let _ = bot.add_issue_comment("b3ww", "vGrant", 9, &open_issue()).await;
        ()
    }
}
