use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{prelude::FromRow, sqlite::SqliteRow, Row, SqlitePool};

use super::{controller::Controller, words::Word};
use sqlx::Column;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct RawCollection {
    id: i64,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Collection {
    id: i64,
    name: String,
    description: String,
    #[sqlx(skip)]
    words: Vec<Word>,
}

pub struct CollectionsController<'a> {
    connection: &'a SqlitePool,
    collection: &'a str,
}

impl<'a> CollectionsController<'a> {
    pub fn new(connection: &'a SqlitePool) -> Self {
        CollectionsController {
            connection,
            collection: "collections",
        }
    }
}

impl Controller<Collection> for CollectionsController<'_> {
    fn get_collection(&self) -> &str {
        self.collection
    }

    fn get_connection(&self) -> &SqlitePool {
        self.connection
    }

    async fn get_all(&self) -> Result<Vec<Collection>, sqlx::Error> {
        let records = sqlx::query(
            "SELECT c.id, c.name, c.description, w.id as word_id, w.word, w.translation, w.image, w.audio FROM collections as c LEFT JOIN collection_words ON c.id = collection_words.collection_id LEFT JOIN words as w ON collection_words.word_id = w.id",
        ).fetch_all(self.connection).await?;

        Ok(collect_collections(records))
    }

    async fn get_one(&self, id: i64) -> Result<Collection, sqlx::Error> {
        let records = sqlx::query(
            "SELECT c.id, c.name, c.description, w.id as word_id, w.word, w.translation, w.image, w.audio FROM collections as c LEFT JOIN collection_words ON c.id = collection_words.collection_id LEFT JOIN words as w ON collection_words.word_id = w.id WHERE c.id = ?",
        ).bind(id).fetch_all(self.connection).await?;

        Ok(collect_collections(records).pop().unwrap())
    }

    async fn create(&self, collection: Value) -> Result<i64, sqlx::Error> {
        let object = collection.as_object().unwrap();

        let query_str = &format!(
            "INSERT INTO collections ({}) VALUES ({})",
            object
                .keys()
                .map(String::as_str)
                .filter(|key| *key != "words")
                .collect::<Vec<_>>()
                .join(", "),
            object
                .keys()
                .filter(|key| *key != "words")
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut record = sqlx::query(query_str);

        for (key, value) in object {
            if value.is_string() && key != "words" {
                record = record.bind(value.as_str().unwrap());
                continue;
            }

            record = record.bind(value);
        }

        let record = record.execute(self.get_connection()).await?;

        let row_id = record.last_insert_rowid();

        if !object.contains_key("words") {
            return Ok(row_id);
        }

        for word in object.get("words").unwrap().as_array().unwrap() {
            let word_id = {
                if word.is_i64() {
                    word.as_i64().unwrap()
                } else {
                    word.as_object()
                        .unwrap()
                        .get("id")
                        .unwrap()
                        .as_i64()
                        .unwrap()
                }
            };

            let _word_record =
                sqlx::query("INSERT INTO collection_words (collection_id, word_id) VALUES (?, ?)")
                    .bind(row_id)
                    .bind(word_id)
                    .execute(self.get_connection())
                    .await?;
        }

        Ok(row_id)
    }

    async fn update(&self, id: i64, item: Value) -> Result<i64, sqlx::Error> {
        let object = item.as_object().unwrap();

        let query_str = &format!(
            "UPDATE collections SET {} WHERE id = ?",
            object
                .keys()
                .filter(|key| *key != "words")
                .map(|key| format!("{} = ?", key))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut record = sqlx::query(query_str);

        for (key, value) in object {
            if value.is_string() {
                record = record.bind(value.as_str().unwrap());
                continue;
            }

            record = record.bind(value);
        }

        record = record.bind(id);

        let record = record.execute(self.get_connection()).await?;

        let mut existing_words = self.get_one(id).await.unwrap().words;

        if !object.contains_key("words") {
            return Ok(id);
        }

        let words = object.get("words").unwrap().as_array().unwrap();

        for word in words {
            let word_id = {
                if word.is_i64() {
                    word.as_i64().unwrap()
                } else {
                    word.as_object()
                        .unwrap()
                        .get("id")
                        .unwrap()
                        .as_i64()
                        .unwrap()
                }
            };

            let existing_word = existing_words.iter().find(|w| w.id == word_id);

            match existing_word {
                // Remove words from existing list to later remove unused
                Some(_) => {
                    existing_words.retain(|w| w.id != word_id);
                }
                // Adds new words to the collection
                None => {
                    let _word_record = sqlx::query(
                        "INSERT INTO collection_words (collection_id, word_id) VALUES (?, ?)",
                    )
                    .bind(id)
                    .bind(word_id)
                    .execute(self.get_connection())
                    .await?;
                }
            }
        }

        // Remove words that are not in the updated collection
        for word in existing_words {
            let _word_record =
                sqlx::query("DELETE FROM collection_words WHERE collection_id = ? AND word_id = ?")
                    .bind(id)
                    .bind(word.id)
                    .execute(self.get_connection())
                    .await?;
        }

        Ok(id)
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let record = sqlx::query("DELETE FROM collections WHERE id = ?")
            .bind(id)
            .execute(self.get_connection())
            .await?;

        Ok(record.rows_affected())
    }
}

fn collect_collections(rows: Vec<SqliteRow>) -> Vec<Collection> {
    let mut collections: Vec<Collection> = vec![];

    for row in rows {
        let id: i64 = row.try_get("id").unwrap();
        let name: String = row.try_get("name").unwrap();
        let description: String = row.try_get("description").unwrap();
        let word_id: i64 = row.try_get("word_id").unwrap();
        let word: String = row.try_get("word").unwrap();
        let translation: String = row.try_get("translation").unwrap();
        let image: String = row.try_get("image").unwrap();
        let audio: String = row.try_get("audio").unwrap();

        let collection = collections.iter_mut().find(|c| c.id == id);

        match collection {
            Some(collection) => {
                collection.words.push(Word {
                    id: word_id,
                    word,
                    translation,
                    image,
                    audio,
                });
            }
            None => {
                collections.push(Collection {
                    id,
                    name: name.clone(),
                    description: description.clone(),
                    words: vec![Word {
                        id: word_id,
                        word,
                        translation,
                        image,
                        audio,
                    }],
                });
            }
        }
    }

    collections
}
