use playwright::{Playwright, api::{ElementHandle, element_handle::{self, ClickBuilder}}};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use playwright::api::{Cookie, ProxySettings, Page};
use actix_web::{get,web, App, HttpServer, post, HttpResponse};


use crate::structs::entry::Entry;
use crate::structs::user::User;
use crate::structs::proxy::Proxy;
use crate::structs::candidate::Candidate;



pub async fn connection(entry: Entry) -> Result<(), playwright::Error> {
    let candidate = Candidate::new(entry.fullname, entry.linkedin, entry.message);

    let proxy = Proxy::new(entry.ip, entry.username, entry.password);

    let user = User::new(entry.user_agent, entry.session_cookie, entry.user_id);

    //proxy settings, TODO add variables instead of fixed values
    let proxy = ProxySettings {
        server: proxy.ip,
        username: Some(proxy.username),
        password: Some(proxy.password),
        bypass:None,
    };

    let playwright = Playwright::initialize().await?;
    playwright.prepare()?; // Install browsers
    let chromium = playwright.chromium();
    let browser = chromium.launcher().proxy(proxy).headless(false).launch().await?;

    //headers, TODO add variable for User-Agent
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), user.user_agent);
    context.set_extra_http_headers(headers).await?;
   
    
    
    //it appears only if you visit the target url, otherwise cookie won't show
    let cookie = Cookie::with_url("li_at", user.session_cookie.as_str(), "https://.www.linkedin.com");
    context.add_cookies(&[cookie]).await?;

    //TODO add variable for the url
    page.goto_builder("https://www.linkedin.com/feed/").goto().await?;
    thread::sleep(Duration::from_secs(3));
    page.wait_for_selector_builder("input[class=search-global-typeahead__input]");
    
    //do some actions
    

    let search_input = page.query_selector("input[class=search-global-typeahead__input]").await?;
    thread::sleep(Duration::from_secs(3));
    //focus on search input and fill it with text
    match search_input {
        Some(search_input) => {
            search_input.hover_builder();
            thread::sleep(Duration::from_secs(1));
            search_input.click_builder().click().await?; // click on search input
            thread::sleep(Duration::from_secs(1)); // wait for 1 second
            search_input.fill_builder(&candidate.fullname).fill().await?; // fill search input with text
            thread::sleep(Duration::from_secs(1));  // wait for 1 second
            search_input.press_builder("Enter").press().await?;  // press Enter
            thread::sleep(Duration::from_secs(5));  // wait for 5 second
            
        },
        None => {
            println!("search_input is None");
        },
    }
    // go to candidate page
    page.goto_builder(candidate.linkedin.as_str()).goto().await?;
    thread::sleep(Duration::from_secs(15));
    //check if connect button is present
    let connect_button = find_button(&page).await;
    match connect_button {
        Ok(_) => message(&page, candidate.message.as_str()).await?,
        Err(_) => {
            thread::sleep(Duration::from_secs(3));
            browser.close().await?;
            return Err(playwright::Error::ObjectNotFound)},
    }
    
    

    thread::sleep(Duration::from_secs(14)); // add delay before closing the browser to check things

    browser.close().await?;
    Ok(())
}



async fn find_button(page: &Page) -> Result<(), playwright::Error> {
    
    // find the block with buttons
    let block = page.query_selector("div[class=pv-top-card-v2-ctas]").await?;
    match block {
        Some(_) => (),
        None => return Err(playwright::Error::ObjectNotFound),
    }
    // find button more actions 
    let more = block.as_ref().unwrap().query_selector("button[aria-label='More actions']").await?;
    match more {
        Some(more) => more.click_builder().click().await?, //click on button more actions
        None => return Err(playwright::Error::ObjectNotFound),
    }
    
    
    thread::sleep(Duration::from_secs(3));
    //find button connect
    let connect = block.unwrap().query_selector("li-icon[type=connect]").await?;
    match connect {
        Some(connect) => connect.click_builder().click().await?, //click on button connect
        None => return Err(playwright::Error::ObjectNotFound),
    }
    
    

    //check if popup to choose "How do you know" appeares
    let popup_how = page.query_selector("button[aria-label='Other']").await?;
    
    match popup_how {
        Some(popup_how) => {
            popup_how.click_builder().click().await?; // click on button "Other"
            // click on button "Connect"
            let connect = page.query_selector("button[aria-label='Connect']").await?;
                match connect {
                    Some(connect) => connect.click_builder().click().await?,
                    None => return Err(playwright::Error::ObjectNotFound),
                }
            }
        ,
        None => ()
    };

    Ok(())
   
}

async fn message(page: &Page, message: &str ) -> Result<(), playwright::Error> {
    //press button add note
    let add_note = page.query_selector("button[aria-label='Add a note']").await?;
    match add_note {
        Some(add_note) => add_note.click_builder().click().await?, // click on button "Other"
        None => return Err(playwright::Error::ObjectNotFound),
        
    };    
    //find input for note
    let text_input = page.query_selector("textarea[id=custom-message]").await?;
    match text_input {
        Some(text_input) => {
            text_input.hover_builder(); // hover on input for note
            thread::sleep(Duration::from_secs(1));
            text_input.focus(); // focus on input for note
            thread::sleep(Duration::from_secs(2));
            text_input.fill_builder(message).fill().await?; // fill input for note;
        },
        None => return Err(playwright::Error::ObjectNotFound),
        
    };

    thread::sleep(Duration::from_secs(1));
    //press button send
    let send = page.query_selector("button[aria-label='Send now']").await?;
    match send {
        Some(send) => send.click_builder().click().await?, // click on button "Send"
        None => return Err(playwright::Error::ObjectNotFound),
        
    };

    Ok(())
}



