use thirtyfour::prelude::*;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenv().ok();
    let mut caps = DesiredCapabilities::chrome();
    //let email = std::env::var("EMAIL");
    //let password = std::env::var("PASSSWORD");
    caps.add_chrome_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Navigate to my profile page
    driver.goto("https://app.solarmoonanalytics.com").await?;
    assert_eq!(driver.title().await?, "Solar Moon Analytics");
    println!("Title = {}", driver.title().await?);

    driver.find(By::ClassName("amplify-image")).await?;

    // Find element from element.
    let email_input = driver.find(By::Id("amplify-id-:r4:")).await?;
    let password_input = driver.find(By::Id("amplify-id-:r7:")).await?;
    

    // Type in email.
    email_input.send_keys().await?;
    // Type in password
    password_input.send_keys().await?;

     // Always explicitly close the browser.
    driver.quit().await?;


    Ok(())
}

