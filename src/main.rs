use futures::future::join_all;
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("https://aetherhub.com/Decks/Historic-Brawl/")
        .await?;

    // Select the group of buttons that let you view decks by time period
    let time_button_group = driver
        .find(By::Css("[aria-label=\"Toolbar with button groups\"]"))
        .await?
        .find_all(By::Tag("label"))
        .await?;

    // Select, then click, the 'year' button to view all decks in the past year
    let year_button = &time_button_group[2];
    year_button.click().await?;

    // Select the table containing the decklists
    let meta_hub_table = driver
        .find(By::Id("metaHubTable"))
        .await
        .expect("Couldn't select meta_hub_table");

    // Select, then click, the decklist table heading "Time since last updated" to sort decks by newest to oldest
    let meta_hub_table_heading_time = meta_hub_table
        .find(By::Tag("thead"))
        .await
        .expect("couldn't find table header")
        .find(By::Css("[title=\"Time since last updated\"]"))
        .await
        .expect("couldn't find the 'time' button");

    meta_hub_table_heading_time
        .click()
        .await
        .expect("couldn't click the 'time' button");

    // meta_hub_table_paginate_next
    //     .wait_until()
    //     .wait(3, 0)
    //     .await
    //     .expect("timed out waiting for 'next page' button to have 'disabled' class");

    // let paginate_next_is_disabled = match meta_hub_table_paginate_next
    //     .class_name()
    //     .await
    //     .expect("couldn't get 'next page' button classname")
    // {
    //     Some(class_name) => class_name.contains("disabled"),
    //     None => false,
    // };

    println!("Got to the loop");

    while true {
        // A Vec of WebElements for each row in the decklist table on the current page
        let meta_hub_table_rows = meta_hub_table
            .find(By::Tag("tbody"))
            .await?
            .find_all(By::Tag("tr"))
            .await?;

        // Collect into a Vec the first 'a' element in each row, which is the decklist url
        let table_decklist_url_futures = meta_hub_table_rows
            .iter()
            .map(|x| async {
                let a_tag = x.find(By::Tag("a")).await.expect("couldn't find 'a' tag");

                let href = a_tag
                    .attr("href")
                    .await
                    .expect("no 'href' attribute on 'a' tag");
                match href {
                    None => String::from("whoops no url"),
                    Some(url) => url,
                }
            })
            .collect::<Vec<_>>();

        // Squish the Vec of futures into one future
        let table_decklist_urls = join_all(table_decklist_url_futures).await;
        // println!("{:#?}", table_decklist_urls);

        let meta_hub_table_paginate_next = driver
            .find(By::Id("metaHubTable_next"))
            .await
            .expect("could't find 'next page' button");

        meta_hub_table_paginate_next
            .click()
            .await
            .expect("couldn't click 'next page' button");
    }

    driver.quit().await?;
    Ok(())
}
