use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub type DbClient = Client<Compat<TcpStream>>;

pub async fn create_connection() -> Result<DbClient, Box<dyn std::error::Error>> {

    println!("Connecting to DB...");

    let mut config = Config::new();

    config.host("SQL1003.site4now.net");
    config.port(1433);
    config.authentication(tiberius::AuthMethod::sql_server(
        "db_ac5b25_qa_admin",
        "SmarterAspNet@12",
    ));

    config.database("db_ac5b25_qa");
    config.encryption(tiberius::EncryptionLevel::Required);
    config.trust_cert();

    println!("Opening TCP...");

    let tcp = TcpStream::connect(config.get_addr()).await?;

    println!("TCP connected");

    tcp.set_nodelay(true)?;

    println!("Connecting client...");

    let client = Client::connect(config, tcp.compat_write()).await?;

    println!("Connected successfully");

    Ok(client)
}