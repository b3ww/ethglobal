use std::collections::HashMap;
use tera::{Context, Tera};

fn generate_template(values: HashMap<String, String>, name: &str) -> String {
    let tera = Tera::new("templates/**/*.tera").expect("Erreur lors du chargement des templates");
    let context = values.iter().fold(Context::new(), |mut ctx, (key, value)| {
        ctx.insert(key, value);
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


mod test {
    use crate::github::Bot;

    use super::close_issue;

    #[tokio::test]
    async fn toto() {
        let bot = Bot::try_new("").unwrap();
        let _ = bot.add_issue_comment("b3ww", "vGrant", 9, &close_issue()).await;
        ()
    }
}
