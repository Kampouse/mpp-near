use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    USDC,
    ZEC,
    BTC,
    ETH,
    SOL,
    NEAR,
}

impl Token {
    pub fn defuse_id(&self) -> &'static str {
        match self {
            Token::USDC => "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
            Token::ZEC => "nep141:zec.omft.near",
            Token::BTC => "nep141:btc.omft.near",
            Token::ETH => "nep141:eth.omft.near",
            Token::SOL => "nep141:sol.omft.near",
            Token::NEAR => "nep141:wrap.near",
        }
    }

    pub fn decimals(&self) -> u8 {
        match self {
            Token::USDC => 6,
            Token::ZEC => 8,
            Token::BTC => 8,
            Token::ETH => 18,
            Token::SOL => 9,
            Token::NEAR => 24,
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "USDC" => Some(Token::USDC),
            "ZEC" => Some(Token::ZEC),
            "BTC" => Some(Token::BTC),
            "ETH" => Some(Token::ETH),
            "SOL" => Some(Token::SOL),
            "NEAR" => Some(Token::NEAR),
            _ => None,
        }
    }

    pub fn to_raw_amount(&self, human: f64) -> String {
        let decimals = self.decimals() as u32;
        let factor = 10u64.pow(decimals);
        let raw = (human * factor as f64) as u128;
        raw.to_string()
    }

    pub fn from_defuse_id(id: &str) -> Option<Self> {
        match id {
            "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1" => Some(Token::USDC),
            "nep141:zec.omft.near" => Some(Token::ZEC),
            "nep141:btc.omft.near" => Some(Token::BTC),
            "nep141:eth.omft.near" => Some(Token::ETH),
            "nep141:sol.omft.near" => Some(Token::SOL),
            "nep141:wrap.near" => Some(Token::NEAR),
            _ => None,
        }
    }
}
