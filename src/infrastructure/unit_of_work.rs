use serde::de::DeserializeOwned;
use serde_json;
use tiberius::Client;
use tiberius::ToSql;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

use crate::infrastructure::database::connection::create_connection;

pub struct UniOfWork {
    pub connection: Option<Client<Compat<TcpStream>>>,
}

impl UniOfWork {
    async fn ensure_connection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.connection.is_none() {
            let conn = create_connection().await?;
            self.connection = Some(conn);
        }
        Ok(())
    }

    pub async fn retrieve_data<T>(
        &mut self,
        query: &str,
        params: &[&dyn ToSql],
    ) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        self.ensure_connection().await?;

        let conn = self.connection.as_mut().unwrap();

        let stream = conn.query(query, params).await?;
        let rows = stream.into_first_result().await?;

        let mut json_result = String::new();

        for row in rows {
            println!(
                "columns: {:?}",
                row.columns().iter().map(|c| c.name()).collect::<Vec<_>>()
            );

            let value: Option<&str> = row.get(0);
            println!("col0 as str = {:?}", value);

            if let Some(v) = value {
                json_result.push_str(v);
            }
        }

        println!("json_result = {}", json_result);

        if json_result.is_empty() {
            json_result = "[]".to_string();
        }

        let data: Vec<T> = serde_json::from_str(&json_result)?;

        Ok(data)
    }

    // pub async fn retrieve_data<T>(&mut self, procedure: &str,) -> Result<Vec<T>, Box<dyn std::error::Error>> where T: DeserializeOwned,
    // {
    //     self.ensure_connection().await?;

    //     let conn = self.connection.as_mut().unwrap();

    //     let query = format!("EXEC {}", procedure);

    //     let stream = conn.simple_query(query).await?;
    //     let rows = stream.into_first_result().await?;

    //     let mut json_result = String::new();

    //     for row in rows {
    //         let value: Option<&str> = row.get(0);
    //         if let Some(v) = value {
    //             json_result.push_str(v);
    //         }
    //     }

    //     if json_result.is_empty() {
    //         json_result = "[]".to_string();
    //     }

    //     let data: Vec<T> = serde_json::from_str(&json_result)?;

    //     Ok(data)
    // }
}
