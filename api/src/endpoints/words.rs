use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    controllers::{
        controller::Controller,
        words::{Word, WordsController},
    },
    AppState,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Option {
    pub name: String,
    pub correct: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Question {
    pub question: String,
    pub options: Vec<Option>,
}

pub async fn get_all_words(State(state): State<Arc<AppState>>) -> Json<Vec<Word>> {
    let controller = WordsController::new(&state.pool);

    let words = controller.get_all().await.unwrap();

    Json(words)
}

pub async fn get_one_word(State(state): State<Arc<AppState>>, Path(id): Path<i64>) -> Json<Word> {
    let controller = WordsController::new(&state.pool);

    let word = controller.get_one(id).await.unwrap();

    Json(word)
}

pub async fn generate_question(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Json<Question> {
    let controller = WordsController::new(&state.pool);

    let word = controller.get_random_word(id).await.unwrap();

    let body = format!(
        r#"{{
  "model": "gpt-4o-2024-08-06",
  "messages": [
    {{
      "role": "system",
      "content": [
        {{
          "type": "text",
          "text": "I'm building a Norwegian learning app. Please suggest me a question related to the word below and three to five other single word options  which the user can choose from. One or more options should be correct. The question should be in English. Don't ask what the word means in English."
        }}
      ]
    }},
    {{
      "role": "user",
      "content": [
        {{
          "type": "text",
          "text": "The word is '{}'"
        }}
      ]
    }}
  ],
  "temperature": 1.52,
  "max_tokens": 2048,
  "top_p": 1,
  "frequency_penalty": 0.61,
  "presence_penalty": 0,
  "response_format": {{
    "type": "json_schema",
    "json_schema": {{
      "name": "response",
      "strict": true,
      "schema": {{
        "type": "object",
        "properties": {{
          "question": {{
            "type": "string"
          }},
          "options": {{
            "type": "array",
            "items": {{
              "type": "object",
              "properties": {{
                "name": {{
                  "type": "string"
                }},
                "correct": {{
                  "type": "boolean"
                }}
              }},
              "additionalProperties": false,
              "required": [
                "name",
                "correct"
              ]
            }}
          }}
        }},
        "additionalProperties": false,
        "required": [
          "question",
          "options"
        ]
      }}
    }}
  }}
}}"#,
        word.word
    );

    println!("{}", body);

    let response = state
        .http_client
        .post("https://api.openai.com/v1/chat/completions")
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let response: Value = serde_json::from_str(&response).unwrap();

    let question = response
        .as_object()
        .unwrap()
        .get("choices")
        .unwrap()
        .as_array()
        .unwrap()[0]
        .as_object()
        .unwrap()
        .get("message")
        .unwrap()
        .as_object()
        .unwrap()
        .get("content")
        .unwrap()
        .as_str()
        .unwrap();

    println!("{}", question);

    let question: Question = serde_json::from_str(question).unwrap();

    Json(question)
}

pub async fn post_word(
    State(state): State<Arc<AppState>>,
    Json(mut word): Json<Value>,
) -> Json<Word> {
    let controller = WordsController::new(&state.pool);

    let id = controller.create(word).await.unwrap();

    let word = controller.get_one(id).await.unwrap();

    Json(word)
}

pub async fn put_word(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(mut word): Json<Value>,
) -> Json<Word> {
    let controller = WordsController::new(&state.pool);

    controller.update(id, word).await.unwrap();

    let word = controller.get_one(id).await.unwrap();

    Json(word)
}

pub async fn delete_word(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<()>, StatusCode> {
    let controller = WordsController::new(&state.pool);

    controller.delete(id).await.unwrap();

    Ok(Json(()))
}
