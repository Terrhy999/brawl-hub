use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://aetherhub.com/Decks/Historic-Brawl/")
        .await?;

    let decklist_time_button_group = driver.find(By::Id("year")).await?;
    decklist_time_button_group.click().await?;

    let decklist_table = driver
        .find(By::Id("metaHubTable"))
        .await?
        .find(By::Tag("tbody"))
        .await?;

    driver.quit().await?;
    Ok(())
}
