use serde::de::DeserializeOwned;
use serde_json;
use tiberius::Client;
use tokio_util::compat::Compat;
use tokio::net::TcpStream;

pub struct UniOfWork {
    pub connection: Client<Compat<TcpStream>>,
}

impl UniOfWork {

    pub async fn retrieve_data<T>(&mut self, procedure: &str,) -> Result<Vec<T>, Box<dyn std::error::Error>> where T: DeserializeOwned,
    {
        let query = format!("EXEC {}", procedure);

        let stream = self.connection.simple_query(query).await?;
        let rows = stream.into_first_result().await?;

        let mut json_result = String::new();

        for row in rows {
            let value: &str = row.get(0).unwrap_or("");
            json_result.push_str(value);
        }

        if json_result.is_empty() {
            json_result = "[]".to_string();
        }

        let data: Vec<T> = serde_json::from_str(&json_result)?;

        Ok(data)
    }
}