use crate::engine::data::item::{ItemMap, Items};
use crate::engine::Hypixel;
use reywen_http::engines::hyper::Method;
use std::collections::HashMap;
impl Hypixel {
    async fn items_get_all(&self) -> reywen_http::engines::hyper::Result<Items> {
        self.engine
            .request(Method::GET, "resources/skyblock/items", None)
            .await
    }
    pub async fn items_get(&self) -> reywen_http::engines::hyper::Result<ItemMap> {
        // let data = self.items_get_all().await?;

        let data = self.items_get_all().await.unwrap();

        let mut map: ItemMap = HashMap::new();
        for item in data.items {
            map.insert(item.id.clone(), item);
        }

        Ok(map)
    }
}

// https://api.hypixel.net/v2/skyblock/bazaar
// https://api.hypixel.net/v2/resources/skyblock/items
