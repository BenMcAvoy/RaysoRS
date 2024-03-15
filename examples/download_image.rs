use rayso_rs::{downloader, RaysoConfig};

const CODE: &str = r#"fn main() {
    println!("Hello, world!");
}"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RaysoConfig::builder()
        .background(false)
        .padding(0)
        .code(CODE)
        .build();

    println!("Generated URL: {}", config.to_url());

    downloader::download(config).await?;

    Ok(())
}
