use lib::oracle::{decode_pyth_price_id, PYTH_PRICE_ID_ETH_USD, PYTH_PRICE_ID_NEAR_USD};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen,
};

#[derive(BorshSerialize, BorshDeserialize, Default, Debug)]
#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    pub fn get_price(
        &self,
        price_identifier: pyth::state::PriceIdentifier,
    ) -> Option<pyth::state::Price> {
        let near_usd = decode_pyth_price_id(PYTH_PRICE_ID_NEAR_USD);
        let eth_usd = decode_pyth_price_id(PYTH_PRICE_ID_ETH_USD);

        if price_identifier.0 == near_usd {
            Some(pyth::state::Price {
                price: 718120242.into(),
                conf: 420242.into(),
                expo: -8,
                publish_time: 1712830518,
            })
        } else if price_identifier.0 == eth_usd {
            Some(pyth::state::Price {
                price: 357262000000.into(),
                conf: 135000000.into(),
                expo: -8,
                publish_time: 1712830748,
            })
        } else {
            None
        }
    }
}
