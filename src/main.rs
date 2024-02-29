use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to my profile page
    driver.goto("https://app.solarmoonanalytics.com").await?;
    assert_eq!(driver.title().await?, "Solar Moon Analytics");
    println!("Title = {}", driver.title().await?);

    driver.find(By::ClassName("amplify-image")).await?;

     // Always explicitly close the browser.
    driver.quit().await?;


    Ok(())
}

