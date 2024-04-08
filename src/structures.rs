#[derive(Debug, Clone, Default)]
pub struct Progress {
    pub height: f32,
    pub width: f32,
    pub percentage: f32,
    pub animated: bool,
}

impl Progress {
    pub fn set(&mut self, height: f32, width: f32, percentage: f32, animated: bool) {
        self.height = height;
        self.width = width;
        self.percentage = percentage;
        self.animated = animated;
    }
    pub fn set_default(&mut self) {
        *self = Self::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchFields {
    pub profit: u32,
    pub order_total: u32,
    pub min_buy_val: u32,
    pub min_sell_val: u32,
    pub name: String,
    pub filter: ItemProperty,
    pub sort_by: SortInfo,
    pub dp: u32,
}

#[derive(Debug, Clone, Default)]
pub struct SortInfo {
    pub sort_by: SortBy,
    pub inverted: bool,
}
#[derive(Debug, PartialEq, Clone, Default)]
pub enum SortBy {
    FlipValue,
    WeeklyOrders,
    #[default]
    Az,
    FlipPercentage,
}

impl ToString for SortBy {
    fn to_string(&self) -> String {
        match self {
            SortBy::FlipValue => "Flip Value",
            SortBy::WeeklyOrders => "Weekly Orders",
            SortBy::Az => "A-Z",
            SortBy::FlipPercentage => "Flip Percentage",
        }
        .to_string()
    }
}
#[derive(Debug, Clone, Default, PartialEq)]
pub enum ItemPropertyF {
    Book,
    Enchanted,
    EnchantedBlock,
    Experience,
    Essence,
    Enchantment,
    #[default]
    Other,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemProperty {
    pub invert: bool,
    pub field: ItemPropertyF,
}

impl ToString for ItemPropertyF {
    fn to_string(&self) -> String {
        if self == &Self::Other {
            "None".to_string()
        } else {
            self.include().into_iter().map(|a| a + " ").collect()
        }
    }
}

impl ItemPropertyF {
    pub fn include(&self) -> Vec<String> {
        match self {
            ItemPropertyF::Book => vec!["book"],
            ItemPropertyF::Enchanted => vec!["enchanted"],
            ItemPropertyF::EnchantedBlock => vec!["enchanted", "block"],
            ItemPropertyF::Experience => vec!["experience"],
            ItemPropertyF::Essence => vec!["essence"],
            ItemPropertyF::Other => Vec::new(),
            ItemPropertyF::Enchantment => vec!["enchantment"],
        }
        .iter()
        .map(|a| a.to_string())
        .collect()
    }
    pub fn exclude(&self) -> Vec<String> {
        match self {
            ItemPropertyF::Enchanted => vec!["block"],
            _ => Vec::new(),
        }
        .iter()
        .map(|a| a.to_string())
        .collect()
    }

    pub fn check2(&self, field: &str) -> bool {
        for include in self.include() {
            if !field.to_ascii_lowercase().contains(&include) {
                return false;
            }
        }
        for exclude in self.exclude() {
            if field.to_ascii_lowercase().contains(&exclude) {
                return false;
            }
        }
        true
    }

    pub fn check3(&self, field: &str, inverted: &bool) -> bool {
        if *inverted {
            !self.check2(field)
        } else {
            self.check2(field)
        }
    }
}
