use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

use crate::with_keys;

use super::controller::Controller;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Word {
    pub id: i64,
    pub word: String,
    pub translation: String,
    pub image: String,
    pub audio: String,
}

// with_keys! {
// pub struct CreateWord {
//     word: String,
//     translation: String,
//     image: String,
//     audio: String,
// }
// }

pub struct WordsController<'a> {
    connection: &'a SqlitePool,
    collection: &'a str,
}

impl<'a> WordsController<'a> {
    pub fn new(connection: &'a SqlitePool) -> Self {
        WordsController {
            connection,
            collection: "words",
        }
    }

    pub async fn get_random_word(&self, id: i64) -> Result<Word, sqlx::Error> {
        let record = sqlx::query_as::<_, Word>("SELECT w.id, w.word, w.translation, w.image, w.audio FROM collections as c LEFT JOIN collection_words ON c.id = collection_words.collection_id LEFT JOIN words as w ON collection_words.word_id = w.id WHERE c.id == ? ORDER BY RANDOM() LIMIT 1").bind(id)
        .fetch_one(self.get_connection())
        .await?;

        Ok(record)
    }
}

impl Controller<Word> for WordsController<'_> {
    fn get_collection(&self) -> &str {
        self.collection
    }

    fn get_connection(&self) -> &SqlitePool {
        self.connection
    }
}
