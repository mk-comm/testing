use playwright::Playwright;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use playwright::api::{Cookie, ProxySettings};



#[tokio::main]
async fn main() -> Result<(), playwright::Error> {


    //proxy settings, TODO add variables instead of fixed values
    let proxy = ProxySettings {
        server: "88.218.148.37:6048".to_owned(),
        username: Some("opncbnxd".to_owned()),
        password: Some("v943hb1fn245".to_owned()),
        bypass:None,
    };

    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().proxy(proxy).headless(false).launch().await?;

   
    //TODO add variable for the url
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://ru.wikipedia.org/wiki/Sefyu").goto().await?;
    
    //it appears only if you visit the target url, otherwise cookie won't show
    let cookie = Cookie::with_url("li_at", "value", "https://.www.example.com");
    context.add_cookies(&[cookie]).await?;


    //do some actions
    

    let search_input = page.query_selector("input[name=search]").await?;
    thread::sleep(Duration::from_secs(3));
    //focus on search input and fill it with text
    match search_input {
        Some(search_input) => {
            search_input.focus().await;
            thread::sleep(Duration::from_secs(1));
            search_input.fill_builder(", this method only performs the [actionability](https://playwright.dev/docs/actionability/) checks and skips the action. Def").fill().await?;
        },
        None => {
            println!("search_input is None");
        },
    }

    
    //headers, TODO add variable for User-Agent
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".to_string());
    context.set_extra_http_headers(headers).await?;
    
    

    thread::sleep(Duration::from_secs(300)); // add delay before closing the browser to check things

    // What this code is doing? looks like it compares current page with example?
    // Exec in browser and Deserialize with serde 
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://example.com/");
    page.click_builder("a").click().await?;
    Ok(())
}