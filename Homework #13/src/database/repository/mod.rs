use sqlx::SqlitePool;

use crate::database::errors::DatabaseErrors;

pub struct SqliteRepository {
    pool: SqlitePool,
}
impl SqliteRepository {
    pub async fn new() -> Result<Self, DatabaseErrors> {
        let url = std::env::var("DATABASE_URL").unwrap();
        match SqlitePool::connect(&url).await {
            Ok(pool) => Ok(SqliteRepository { pool }),
            Err(error) => Err(DatabaseErrors::ConnectionError(error.to_string())),
        }
    }
    pub async fn add_room(&mut self, room_name: &str) -> Result<u32, DatabaseErrors> {
        let mut connection = match self.pool.acquire().await {
            Ok(connection) => connection,
            Err(error) => return Err(DatabaseErrors::ConnectionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO rooms ( name )
            VALUES ( ?1 )
            "#,
            room_name
        )
        .execute(&mut connection)
        .await;

        match result {
            Err(error) => Err(DatabaseErrors::RequestError(error.to_string())),
            Ok(query_result) => Ok(query_result.last_insert_rowid() as u32),
        }
    }
    pub async fn delete_room(&mut self, room_name: &str) -> Result<(), DatabaseErrors> {
        let mut connection = match self.pool.acquire().await {
            Ok(connection) => connection,
            Err(error) => return Err(DatabaseErrors::ConnectionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            DELETE FROM rooms 
            WHERE name = ?1 
            "#,
            room_name
        )
        .execute(&mut connection)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }
        Ok(())
    }
    pub async fn add_device(
        &mut self,
        room_id: u32,
        device_name: &str,
    ) -> Result<(), DatabaseErrors> {
        let mut connection = match self.pool.acquire().await {
            Ok(connection) => connection,
            Err(error) => return Err(DatabaseErrors::ConnectionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO devices_rooms ( room_id, device_name )
            VALUES ( ?1, ?2 )
            "#,
            room_id,
            device_name
        )
        .execute(&mut connection)
        .await;

        match result {
            Err(error) => Err(DatabaseErrors::RequestError(error.to_string())),
            Ok(_) => Ok(()),
        }
    }
    pub async fn delete_device(
        &mut self,
        room_id: u32,
        device_name: &str,
    ) -> Result<(), DatabaseErrors> {
        let mut connection = match self.pool.acquire().await {
            Ok(connection) => connection,
            Err(error) => return Err(DatabaseErrors::ConnectionError(error.to_string())),
        };

        let result = sqlx::query!(
            r#"
            DELETE FROM devices_rooms 
            WHERE room_id = ?1 AND device_name = ?2
            "#,
            room_id,
            device_name
        )
        .execute(&mut connection)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }
        Ok(())
    }
    pub async fn get_rooms(&self) -> Result<Vec<String>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT name FROM rooms 
            "#
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        Ok(result
            .unwrap()
            .iter()
            .map(|record| record.name.to_string())
            .collect())
    }
    pub async fn get_devices(&self, room_name: &str) -> Result<Vec<String>, DatabaseErrors> {
        let result = sqlx::query!(
            r#"
            SELECT device_name FROM devices_rooms
            WHERE room_id IN (SELECT id FROM rooms WHERE name = ?1) 
            "#,
            room_name
        )
        .fetch_all(&self.pool)
        .await;

        if let Err(error) = result {
            return Err(DatabaseErrors::RequestError(error.to_string()));
        }

        Ok(result
            .unwrap()
            .iter()
            .map(|record| record.device_name.to_string())
            .collect())
    }
}
