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

fn m() -> String {
    generate_template(HashMap::from([
        ("grant_creator_address".to_string(), "0xABC123...DEF".to_string()),
        ("grant_amount".to_string(), "100.00".to_string()),
        ("contract_address".to_string(), "0xCONTRACT123...XYZ".to_string()),
    ]), "open_issue.tera")
}

mod test {
    use crate::github::Bot;

    use super::m;

    #[tokio::test]
    async fn toto() {
        let bot = Bot::try_new("").unwrap();
        let _ = bot.add_issue_comment("b3ww", "vGrant", 9, &m()).await;
        ()
    }
}
