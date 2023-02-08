use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    // ... (outros campos omitidos)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get("https://api.github.com/users/PierreOliveira1").header("User-Agent", "rust").send().await?;
    let body = res.text().await?;

    let user: User = serde_json::from_str(&body)?;

    println!("User name: {}", user.login);

    Ok(())
}
