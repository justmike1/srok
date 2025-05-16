use std::sync::OnceLock;

pub const ROWS_PER_PAGE: usize = 100;

pub static ORIGIN_BASE_URL: OnceLock<String> = OnceLock::new();

pub static GITHUB_TOKEN: OnceLock<String> = OnceLock::new();

pub static SHODAN_TOKEN: OnceLock<String> = OnceLock::new();

pub fn get_origin_base_url() -> &'static str {
    ORIGIN_BASE_URL.get_or_init(|| {
        std::env::var("ORIGIN_BASE_URL")
            .or_else(|_| std::env::var("LEPTOS_SITE_ADDR"))
            .unwrap_or_else(|_| {
                panic!("Neither ORIGIN_BASE_URL nor LEPTOS_SITE_ADDR environment variables are set")
            })
    })
}

pub fn get_github_token() -> &'static str {
    GITHUB_TOKEN.get_or_init(|| {
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable not set")
    })
}

pub fn get_shodan_token() -> &'static str {
    SHODAN_TOKEN.get_or_init(|| {
        std::env::var("SHODAN_API_KEY").expect("SHODAN_API_KEY environment variable not set")
    })
}
