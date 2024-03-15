use thirtyfour::{common::capabilities::firefox::FirefoxPreferences, prelude::*};

use crate::RaysoConfig;

use std::{env, thread::{self, sleep}, time::Duration};

pub async fn download(config: RaysoConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut caps = DesiredCapabilities::firefox();
    let mut prefs = FirefoxPreferences::new();

    // #[cfg(not(debug_assertions))]
    caps.set_headless()?;

    // Set download directory to current working directory
    let dir = env::current_dir()?.to_str().unwrap().to_string();
    prefs.set("browser.download.folderList", 2)?;
    prefs.set("browser.download.dir", dir)?;
    caps.set_preferences(prefs)?;

    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    driver.goto(config.to_url()).await?;

    thread::sleep(Duration::from_secs(2));

    // TODO: Don't hardcode the class name.
    let export_button = driver
        .query(By::ClassName("ExportButton_button__MA4PI"))
        .first()
        .await?;

    export_button.click().await?;

    // Sleep to allow processing of the download
    sleep(Duration::from_secs(5));

    driver.quit().await?;

    Ok(())
}
