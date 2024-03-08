# RaysoRS

## Dependencies:
- [GeckoDriver (external)](https://github.com/mozilla/geckodriver)
- [ThirtyFour (crate)](https://crates.io/crates/thirtyfour)
- [Tokio (crate)](https://crates.io/crates/tokio)
- [Base64 (crate)](https://crates.io/crates/base64)
- [URL encoding (crate)](https://crates.io/crates/urlencoding)

## Usage
Make sure you are running `geckodriver` in the background if you are doing downloads as this project uses the geckodriver webdriver.

You can use the library as follows:
```rust
use rayso_rs::{downloader, RaysoConfig};

const CODE: &str = r#"fn main() {
    println!("Hello, world!");
}"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RaysoConfig::builder()
        .background(false)
        .padding(8)
        .code(CODE)
        .build();

    println!("Generated URL: {}", config.to_url());

    downloader::download(config).await?;

    Ok(())
}
```

Note: the `downloader::download(config)`... is the only part that requires geckodriver to be running in the background.
