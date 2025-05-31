use tera::{Tera, Context};

pub fn m() {
    let tera = Tera::new("templates/close_issue.tera").expect("Failed to load templates");

    let mut context = Context::new();
    context.insert("contract_address", "0xCONTRACT123...XYZ");
    context.insert("grant_amount", "100.00");
    context.insert("developer_name", "vitalik");
    context.insert("developer_address", "0xABC123...DEF");
    context.insert("gas_fee", "0.1");
    context.insert("tx_hash", "0x12345...XYZ");

    let rendered = tera.render("grant_comment.tera", &context)
        .expect("Failed to render template");

    println!("{}", rendered);
}
