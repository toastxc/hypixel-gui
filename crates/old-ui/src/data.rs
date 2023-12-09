use druid::{Data, Env, EventCtx, Lens, Widget};
use engine::{methods::bazaar::ProfitInfo, Hypixel};
use std::num::{ParseFloatError, ParseIntError};
use std::sync::{mpsc, Arc};
use druid::im::Vector;
use druid::widget::Flex;
use tokio::runtime::Runtime;

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

impl From<engine::Error> for Error {
    fn from(value: engine::Error) -> Self {
        Self::HyperError(value)
    }
}
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    HyperError(engine::Error),
}


#[derive(Clone, Lens, Debug)]
pub struct AppState {
    pub order_total: String,
    pub price_diff: String,
    pub original_data: Vec<ProfitInfo>,
    pub processed_data: Vec<ProfitInfo>,
    pub runtime: Arc<Runtime>,
}

impl AppState {
    pub fn click_add_find_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let (sender, receiver) = mpsc::channel();

        let runtime_engine = data.runtime.clone();

        runtime_engine.spawn(async move {
            let now = std::time::SystemTime::now();

            let a = Hypixel::new().bazaar_profit().await.unwrap();

            sender.send(a).unwrap();

            let done = std::time::SystemTime::now().duration_since(now);
            println!("request elapsed: {:?}", done);
        });

        data.original_data = receiver.recv().unwrap();
    }

    pub async fn find_calculate(&mut self) -> Result<Vec<ProfitInfo>, Error> {
        let a = Hypixel::new().bazaar_profit().await?;
        self.original_data = a.clone();

        Ok(a)
    }

    pub fn click_update_view(_ctx: &mut EventCtx, _data: &mut Self, _env: &Env) {}

    pub fn click_calculate_button(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        let polled_data = data.clone().original_data;
        // filtering
        let mut polled_data: Vec<ProfitInfo> = polled_data
            .into_iter()
            .filter(|a| {
                a.bazaar_buy_price != 0.0
                    && a.weekly_buy_orders != 0
                    && a.bazaar_buy_price > a.bazaar_sell_price
                    && a.flip_value > data.price_diff.parse().unwrap_or_default()
                    && { a.weekly_buy_orders + a.weekly_sell_orders }
                        > data.order_total.parse().unwrap_or_default()
            })
            .collect();

        // sorted
        polled_data.sort_by_key(|item| {
            (item.flip_value as i32, {
                item.weekly_buy_orders + item.weekly_sell_orders
            })
        });

       // data.processed_data = polled_data;



    }

    pub fn data(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) -> Self {
        data.to_owned()
    }
}
