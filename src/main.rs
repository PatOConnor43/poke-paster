use std::{collections::BTreeSet, error::Error};

use reqwest::Url;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let r = regex_lite::Regex::new(r"https://pokepast\.es/([0-9a-f]{16})")?;
    let url = Url::parse("https://old.reddit.com/r/stunfisk/search/?q=pokepast.es&sort=new")?;
    let client = reqwest::blocking::ClientBuilder::default()
        .user_agent(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/116.0",
        )
        .build()?;
    let response = client.get(url.clone()).send()?.text()?;
    let mut paste_ids: BTreeSet<String> = BTreeSet::new();
    for caps in r.captures_iter(&response) {
        if let Some(id) = caps.get(1) {
            paste_ids.insert(id.as_str().to_string());
        }
    }

    let mut existing_ids = std::fs::read_to_string("ids")?
        .split("\n")
        .filter(|id| !id.is_empty())
        .map(|id| id.to_string())
        .collect::<BTreeSet<String>>();

    existing_ids.extend(paste_ids);
    std::fs::write(
        "ids",
        existing_ids
            .iter()
            .fold(String::new(), |a, b| format!("{}{}\n", a, b)),
    )?;

    Ok(())
}
