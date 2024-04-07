use reywen_http::engines::hyper::*;

#[derive(Debug, Clone, Default)]
pub struct Hypixel {
    pub api_key: Option<String>,
    pub engine: Hyper,
}

pub use results::Error;
pub mod data;
pub mod methods;

const URL: &str = "https://api.hypixel.net/v2/";

impl Hypixel {
    pub fn new() -> Self {
        Self {
            api_key: None,
            engine: Hyper::new().set_url(URL),
        }
    }
    pub fn new_with_key(api_key: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            engine: Hyper::new().set_url(URL),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_bazaar() {
        let a = Hypixel::new("").bazaar_get().await.unwrap();

        panic!("{:?}", a);
    }

    #[tokio::test]
    async fn test_data_bazaar() {
        let a = Hypixel::new("")
            .bazaar_profit(100000000, 1.0)
            .await
            .unwrap();

        panic!("{:?}", a);
    }
}
