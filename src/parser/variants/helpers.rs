use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{Browser, LaunchOptions};
use std::ffi::OsStr;
use std::time::Duration;

pub async fn get_browser(headless: bool) -> Result<Browser, failure::Error> {
    const WAITING_TIMEOUT_SEC: u64 = 30;

    let _launch_options = LaunchOptions {
        enable_logging: true,
        headless,
        sandbox: true,
        enable_gpu: false,
        window_size: Some((400, 400)),
        port: None,
        ignore_certificate_errors: true,
        path: None,
        user_data_dir: None,
        extensions: [].to_vec(),
        args: [OsStr::new("--incognito")].to_vec(),
        fetcher_options: Default::default(),
        disable_default_args: false,
        idle_browser_timeout: Duration::from_secs(WAITING_TIMEOUT_SEC),
        process_envs: None,
        proxy_server: None,
    };
    let browser = Browser::new(_launch_options).unwrap();
    Ok(browser)
}
pub fn log_tabs(browser: &Browser) {
    let tabs = browser.get_tabs().lock().unwrap().clone();

    tabs.into_iter().for_each(|tab| {
        println!("Tab {:?}", tab.get_url());
    });
}

pub async fn new_incognito_tab(
    browser: &Browser,
    url: String,
) -> Result<std::sync::Arc<headless_chrome::Tab>, failure::Error> {
    // let browser_context_id = Some(browser.new_context().unwrap().get_id().to_string());
    let params = CreateTarget {
        url,
        width: None,
        height: None,
        browser_context_id: None,
        enable_begin_frame_control: Some(false),
        new_window: Some(true),
        background: Some(true),
    };
    let tab: std::sync::Arc<headless_chrome::Tab> = browser.new_tab_with_options(params).unwrap();
    Ok(tab)
}

pub fn get_src_set(src_set: &str) -> Result<&str, failure::Error> {
    let mut variants: Vec<&str> = src_set.split(", ").collect();
    let last_option: Vec<&str> = variants.pop().unwrap().split(" ").collect();
    let image_url = last_option[0];
    Ok(image_url)
}

#[tokio::test]
async fn get_src_set_test() {
    let src_set =
    "https://s.hdnux.com/photos/01/35/60/77/24580068/6/80x0.jpg 80w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/160x0.jpg 160w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/240x0.jpg 240w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/360x0.jpg 360w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/480x0.jpg 480w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/640x0.jpg 640w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/720x0.jpg 720w, https://s.hdnux.com/photos/01/35/60/77/24580068/6/960x0.jpg 960w";
    let image_url = get_src_set(src_set).unwrap();

    assert_eq!(
        image_url,
        String::from("https://s.hdnux.com/photos/01/35/60/77/24580068/6/960x0.jpg")
    );
}
