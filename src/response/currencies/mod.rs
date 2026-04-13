use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{EnumIter, EnumString, IntoEnumIterator};

#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    pub currencies: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullCurrencies {
    pub currencies: Vec<SingleCurrency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleCurrency {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: bool,
    pub wallet_regex: String,
    pub priority: i64,
    pub extra_id_exists: bool,
    pub extra_id_regex: Option<String>,
    pub logo_url: String,
    pub track: bool,
    pub cg_id: String,
    pub is_maxlimit: bool,
    pub network: Option<String>,
    pub smart_contract: Option<String>,
    pub network_precision: Option<String>,
    pub explorer_link_hash: Option<String>,
    pub precision: i64,
    pub ticker: Option<String>,
    pub is_defi: bool,
    pub is_popular: bool,
    pub is_stable: bool,
    pub available_for_to_conversion: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectedCurrencies {
    #[serde(rename = "selectedCurrencies")]
    pub selected_currencies: Vec<String>,
}

/// TODO: add every currency supported
/// The enum name is the currency "code"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumIter, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Currency {
    // Real money
    XMR,

    // Altcoins / crypto
    BTC,
    ETH,
    SOL,
    TRX,
    BNBBSC,

    // Stablecoins
    USDT,
    USDC,
    USDCSOL,
    USDTERC20,
    USDTTRC20,
    USDTBSC,

    // Fiat
    USD,

    UNKNOWN,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Methods
impl Currency {
    /// The official currency name.
    pub fn name(&self) -> String {
        match self {
            Self::XMR => "Monero",
            Self::BTC => "Bitcoin",
            Self::ETH => "Ethereum",
            Self::SOL => "Solana",
            Self::TRX => "Tron",
            Self::BNBBSC => "BNB (Binance Smart Chain)",

            Self::USDT => "Tether USD",
            Self::USDC => "USD Coin",
            Self::USDCSOL => "USD Coin (Solana)",
            Self::USDTERC20 => "Tether USD (ERC20)",
            Self::USDTTRC20 => "Tether USD (TRC20)",
            Self::USDTBSC => "Tether USD (BEP20)",

            Self::USD => "US Dollar",

            Self::UNKNOWN => "Unknown",
        }
        .to_string()
    }

    /// The CoinGecko / NOWPayments ID
    pub fn cg_id(&self) -> String {
        match self {
            Self::XMR => "monero",
            Self::BTC => "bitcoin",
            Self::ETH => "ethereum",
            Self::SOL => "solana",
            Self::TRX => "tron",
            Self::BNBBSC => "binancecoin",

            Self::USDT => "tether",
            Self::USDC => "usd-coin",
            Self::USDCSOL => "usd-coin",
            Self::USDTERC20 => "tether",
            Self::USDTTRC20 => "tether",
            Self::USDTBSC => "tether",

            Self::USD => "usd",

            Self::UNKNOWN => "unknown",
        }
        .to_string()
    }

    /// The currency network (used for URI schemes)
    pub fn network(&self) -> String {
        match self {
            Self::XMR => "xmr",
            Self::BTC => "btc",
            Self::ETH => "eth",
            Self::SOL => "sol",
            Self::TRX => "trx",
            Self::BNBBSC => "bsc",

            Self::USDT => "eth", // default fallback
            Self::USDC => "eth",

            Self::USDCSOL => "sol",
            Self::USDTERC20 => "eth",
            Self::USDTTRC20 => "trx",
            Self::USDTBSC => "bsc",

            Self::USD => "fiat",

            Self::UNKNOWN => "unknown",
        }
        .to_string()
    }

    pub fn protocol(&self) -> &'static str {
        match self {
            // Native coins (no token standard)
            Self::BTC => "native",
            Self::ETH => "native",
            Self::XMR => "native",
            Self::SOL => "native",
            Self::TRX => "native",
            Self::BNBBSC => "native",

            // Stablecoins / tokens
            Self::USDTERC20 => "erc20",
            Self::USDTTRC20 => "trc20",
            Self::USDTBSC => "bep20",

            Self::USDCSOL => "spl",
            Self::USDC => "erc20", // default assumption

            Self::USDT => "erc20", // default fallback unless specified

            // Fiat
            Self::USD => "fiat",

            Self::UNKNOWN => "unknown",
        }
    }
}

// Getters
impl Currency {
    pub fn get_stablecoins() -> Vec<Currency> {
        let items: Vec<Currency> = Self::get_all()
            .iter()
            .filter(|e| e.to_string().starts_with("USD"))
            .map(|e| e.to_owned())
            .collect();
        items
    }
    pub fn get_all() -> Vec<Currency> {
        let items: Vec<Currency> = Currency::iter()
            // Remove fiat and convenience enum variant.
            .filter(|e| !vec![Currency::USD, Currency::UNKNOWN].contains(e))
            .collect();
        items
    }
}
