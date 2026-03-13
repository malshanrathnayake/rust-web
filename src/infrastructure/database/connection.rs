use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type DbClient = Client<Compat<TcpStream>>;

pub async fn create_connection() -> Result<DbClient, Box<dyn std::error::Error>> {

    let mut config = Config::new();

    config.host("SQL9001.site4now.net");
    config.port(1433);
    config.authentication(tiberius::AuthMethod::sql_server(
        "db_ac3eca_sensei_admin",
        "smarterAspNet@12",
    ));

    config.database("db_ac3eca_sensei");
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await?;

    Ok(client)
}