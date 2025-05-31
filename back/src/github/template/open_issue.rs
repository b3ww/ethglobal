use tera::{Tera, Context};

pub fn m() {
    let tera = Tera::new("templates/open_issue.tera").expect("Failed to load templates");

    let mut context = Context::new();
    context.insert("grant_creator_address", "0xABC123...DEF");
    context.insert("grant_amount", "100.00");
    context.insert("contract_address", "0xCONTRACT123...XYZ");

    let rendered = tera.render("grant_comment.tera", &context)
        .expect("Failed to render template");

    println!("{}", rendered);
}
