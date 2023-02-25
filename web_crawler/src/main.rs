use playwright::Playwright;
use std::fs::read_to_string;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use playwright::api::Cookie;


#[tokio::main]
async fn main() -> Result<(), playwright::Error> {
    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(false).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://linkedin.com").goto().await?;
    

    //it appears only if you visit the target url, otherwise cookie won't show
    let cookie = Cookie::with_url("li_at", "value", "https://.www.linkedin.com");
   
    
    
    context.add_cookies(&[cookie]).await?; // does not work 
    
    let mut headers = HashMap::new();
    //headers.insert("cookie".to_string(), "li_at=123".to_string()); // should be deleted, useless
    //headers.insert("cookie".to_string(), "JSESSIONID=typebit".to_string()); // should be deleted, useless
    headers.insert("User-Agent".to_string(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".to_string());
    context.set_extra_http_headers(headers).await?;

    

    thread::sleep(Duration::from_secs(300)); // add delay before closing the browser to check things

    // Exec in browser and Deserialize with serde // What this code is doing? looks like it compares current page with example?
    let s: String = page.eval("() => location.href").await?;
    assert_eq!(s, "https://example.com/");
    page.click_builder("a").click().await?;
    Ok(())
}