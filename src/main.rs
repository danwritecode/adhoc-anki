use anyhow::{Result, bail};
use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rand::seq::SliceRandom;
use rand::thread_rng;


const DECK_NAME: &str = "Grokking Networking";
const DECK_FORMAT: &str = "Study Multiple Choice";

#[tokio::main]
async fn main() -> Result<()> {
    const FILE_CONTENTS: &str = include_str!("../networks_osi.json");
    let questions: Vec<Question> = serde_json::from_str(FILE_CONTENTS)?;

    for (i, q) in questions.iter().enumerate() {
        println!("Processing: {}", i);
        
        let question = q.question.clone();
        let answer = q.answers[q.answer_index].clone();
        
        let mut shuffled_answers = q.answers.clone();
        shuffled_answers.shuffle(&mut thread_rng());

        let answer_a = shuffled_answers[0].clone();
        let answer_b = shuffled_answers[1].clone();
        let answer_c = shuffled_answers[2].clone();
        let answer_d = shuffled_answers[3].clone();

        add_card_anki_multiple_choice(&question, &answer_a, &answer_b, &answer_c, &answer_d, &answer).await?;
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

async fn add_card_anki_multiple_choice(
    question: &str,
    answer_a: &str,
    answer_b: &str,
    answer_c: &str,
    answer_d: &str,
    answer: &str
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
                        "Question": question,
                        "AnswerA": answer_a,
                        "AnswerB": answer_b,
                        "AnswerC": answer_c,
                        "AnswerD": answer_d,
                        "Answer": answer
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
                        "networking"
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub answer_index: usize,
}
