use anyhow::{Result, bail};
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Value;

const DECK_NAME: &str = "U Verbs";
const DECK_FORMAT: &str = "Basic (type in the answer)";

#[tokio::main]
async fn main() -> Result<()> {
    const FILE_CONTENTS: &str = include_str!("../words.json");
    let words: Vec<Word> = serde_json::from_str(FILE_CONTENTS)?;

    for w in &words {
        let word_plain = format!("Plain: {}", w.verb);
        let word_polite = format!("Polite: {}", w.verb);

        add_card_anki(&word_plain, &w.past_plain).await?;
        add_card_anki(&word_polite, &w.past_polite).await?;
    }

    Ok(())
}

async fn add_card_anki(
    front: &str,
    back: &str
) -> Result<()> {
    let res: Value = Client::new()
        .post("http://localhost:8765")
        .json(&serde_json::json!({
            "action": "addNote",
            "version": 6,
            "params": {
                "note": {
                    "deckName": DECK_NAME,
                    "modelName": DECK_FORMAT,
                    "fields": {
                        "Front": front,
                        "Back": back
                    },
                   "options": {
                        "allowDuplicate": false,
                        "duplicateScope": "deck",
                        "duplicateScopeOptions": {
                            "deckName": "Default",
                            "checkChildren": false,
                            "checkAllModels": false
                        }
                    },
                    "tags": [
                        "u-verbs"
                    ]
                } 
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("res: {:#?}", res);

    if !res["error"].is_null() {
        bail!("Response from anki contained error(s) | error(s): {}", res["error"]) 
    }

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Word {
    pub verb: String,
    pub meaning: String,
    pub past_plain: String,
    pub past_polite: String,
}
