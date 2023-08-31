use rusqlite::Result;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

use crate::models::{
    color::Color,
    color_input::ColorInput,
    phone::{Phone, PhoneType},
    phone_input::PhoneInput,
    user::User,
};

const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone)]
pub struct Database {
    pub conn_ref: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Database> {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        };

        let db = SqlitePool::connect(DB_URL).await.unwrap();
        Ok(Database { conn_ref: db })
    }
    pub async fn get_user(&self, id: &i32) -> Option<User> {
        let db = &self.conn_ref;
        sqlx::query!(
            r#"
                SELECT users.id, users.name, users.color_id, user_colors.color_name, users.phone_id, phones.number, phones.phone_type
                FROM users
                INNER JOIN user_colors
                    ON users.color_id = user_colors.id
                    INNER JOIN phones
                    ON users.phone_id = phones.id
                WHERE users.id = ?"#
            , id)
            .fetch_one(db)
            .await
            .ok()
            .map(|row| User {
                id: row.id as i32,
                name: row.name.unwrap(),
                color: Color {
                    id: row.color_id.unwrap() as i32,
                    name: row.color_name,
                },
                phone: Phone {
                    id: row.phone_id.unwrap() as i32,
                    number: row.number.unwrap(),
                    phone_type: {
                        let x: String = row.phone_type.unwrap();
                        PhoneType::from(x)
                    },
                },
            })
    }

    pub async fn get_users(&self) -> Vec<User> {
        let db = &self.conn_ref;
        sqlx::query!(
            r#"
                SELECT users.id, users.name, users.color_id, user_colors.color_name, users.phone_id, phones.number, phones.phone_type
                FROM users
                INNER JOIN user_colors
                    ON users.color_id = user_colors.id
                    INNER JOIN phones
                    ON users.phone_id = phones.id"#
            ).fetch_all(db).await.unwrap().into_iter().map(|row| User {
                id: row.id as i32,
                name: row.name.unwrap(),
                color: Color {
                    id: row.color_id.unwrap() as i32,
                    name: row.color_name,
                },
                phone: Phone {
                    id: row.phone_id.unwrap() as i32,
                    number: row.number.unwrap(),
                    phone_type: {
                        let x: String = row.phone_type.unwrap();
                        PhoneType::from(x)
                    },
                },
            }).collect()
    }

    pub async fn get_colors(&self) -> Vec<Color> {
        let binding = &self.conn_ref;
        sqlx::query!(r#"SELECT id, color_name FROM user_colors"#l)
            .fetch_all(binding)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Color {
                id: row.id as i32,
                name: row.color_name,
            })
            .collect()
    }

    pub async fn get_user_count(&self) -> i32 {
        let db = &self.conn_ref;
        sqlx::query!(r#"SELECT COUNT(id) as count FROM users;"#)
            .fetch_one(db)
            .await
            .unwrap()
            .count
    }

    pub async fn delete_user(&self, id: &i32) -> Result<(), String> {
        let db = &self.conn_ref;
        sqlx::query!(
            r#"
                DELETE FROM users
                WHERE id = ?"#,
            id
        )
        .execute(db)
        .await
        .unwrap();
        Ok(())
    }

    pub async fn add_user(
        &self,
        user: &String,
        phone: &PhoneInput,
        color: &ColorInput,
    ) -> Option<i32> {
        let db = &self.conn_ref;
        let phone_type = phone.phone_type.to_string();
        let mut tx = db.begin().await.unwrap();
        let Ok(c) = sqlx::query!(
            r#"
                INSERT INTO user_colors (color_name)
                VALUES (?)
                RETURNING id"#,
            color.name
        )
        .fetch_one(&mut *tx)
        .await else {
            tx.rollback().await.unwrap();
            return None;
        };
        let Ok(p) = sqlx::query!(
            r#"
                INSERT INTO phones (number, phone_type)
                VALUES (?, ?)
                RETURNING id"#,
            phone.number,
            phone_type,
        )
        .fetch_one(&mut *tx)
        .await else {
            tx.rollback().await.unwrap();
            return None
         };
        let Ok(u) = sqlx::query!(
            r#"
                INSERT INTO users (name, color_id, phone_id)
                VALUES (?, ?, ?)
                RETURNING id"#,
            user,
            c.id,
            p.id,
        )
        .fetch_one(&mut *tx)
        .await else {
            tx.rollback().await.unwrap();
            return None
        };
        tx.commit().await.unwrap();
        Some(u.id as i32)
    }
}
