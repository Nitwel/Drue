use axum::Json;
use serde_json::Value;
use sqlx::{query, query_as, Database, FromRow, Sqlite, SqlitePool};

pub trait Controller<Item>
where
    Item: Send + Unpin,
    Item: for<'r> FromRow<'r, <Sqlite as Database>::Row>,
{
    fn get_collection(&self) -> &str;
    fn get_connection(&self) -> &SqlitePool;

    async fn get_all(&self) -> Result<Vec<Item>, sqlx::Error> {
        let records = query_as::<_, Item>(&format!("SELECT * FROM {}", self.get_collection()))
            .fetch_all(self.get_connection())
            .await?;

        Ok(records)
    }

    async fn get_one(&self, id: i64) -> Result<Item, sqlx::Error> {
        let record = query_as::<_, Item>(&format!(
            "SELECT * FROM {} WHERE id = ?",
            self.get_collection()
        ))
        .bind(id)
        .fetch_one(self.get_connection())
        .await?;

        Ok(record)
    }

    async fn create(&self, item: Value) -> Result<i64, sqlx::Error> {
        let object = item.as_object().unwrap();

        let query_str = &format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.get_collection(),
            object
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", "),
            object.keys().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut record = query(query_str);

        for (key, value) in object {
            if value.is_string() {
                record = record.bind(value.as_str().unwrap());
                continue;
            }

            record = record.bind(value);
        }

        let record = record.execute(self.get_connection()).await?;

        Ok(record.last_insert_rowid())
    }

    async fn update(&self, id: i64, item: Value) -> Result<i64, sqlx::Error> {
        let object = item.as_object().unwrap();

        let query_str = &format!(
            "UPDATE {} SET {} WHERE id = ?",
            self.get_collection(),
            object
                .iter()
                .map(|(key, value)| format!("{} = ?", key))
                .collect::<Vec<String>>()
                .join(", ")
        );

        let mut record = query(query_str);

        for (key, value) in object {
            if value.is_string() {
                record = record.bind(value.as_str().unwrap());
                continue;
            }

            record = record.bind(value);
        }

        record = record.bind(id);

        let record = record.execute(self.get_connection()).await?;

        Ok(id)
    }

    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error> {
        let record = query(&format!(
            "DELETE FROM {} WHERE id = ?",
            self.get_collection()
        ))
        .bind(id)
        .execute(self.get_connection())
        .await?;

        Ok(record.rows_affected())
    }
}

#[macro_export]
macro_rules! create_controller {
    ($name: ident, $item: ty) => {
        use sqlx::{Database, FromRow, Sqlite, SqlitePool};

        pub struct $name<'a> {
            connection: &'a SqlitePool,
            collection: &'a str,
        }

        impl<'a> $name<'a> {
            pub fn new(connection: &'a SqlitePool, collection: &'a str) -> Self {
                $name {
                    connection,
                    collection,
                }
            }
        }
    };
}

// #[macro_export]
// macro_rules! create_controller {
//     ($name: ident, $item: ty $(, $fn:ident)*) => {
//         use sqlx::{Database, FromRow, Sqlite, SqlitePool};

//         pub struct $name<'a> {
//             connection: &'a SqlitePool,
//             collection: &'a str,
//         }

//         impl<'a> $name<'a> {
//             pub fn new(connection: &'a SqlitePool, collection: &'a str) -> Self {
//                 $name {
//                     connection,
//                     collection,
//                 }
//             }

//             // only add get_all if it is in the list of functions

//             $(
//                 #[cfg(get_all)]
//                 pub async fn get_all(&self) -> Result<Vec<$item>, sqlx::Error> {
//                     let records =
//                         sqlx::query_as::<_, $item>(&format!("SELECT * FROM {}", self.collection))
//                             .fetch_all(self.connection)
//                             .await?;

//                     Ok(records)
//                 }
//             )*
//         }
//     };
// }
