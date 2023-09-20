use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://aetherhub.com/Decks/Historic-Brawl/")
        .await?;

    let time_button_group = driver
        .find(By::Css("[aria-label=\"Toolbar with button groups\"]"))
        .await?
        .find_all(By::Tag("label"))
        .await?;

    let year_button = &time_button_group[2];
    year_button.click().await?;

    println!("{}", year_button.outer_html().await?);

    driver.quit().await?;
    Ok(())
}
