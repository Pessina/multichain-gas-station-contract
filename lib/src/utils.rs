use near_sdk::{env, require, NearToken};

pub fn assert_gt_one_yocto_near() {
    require!(
        env::attached_deposit() >= NearToken::from_yoctonear(1),
        "Requires attached deposit of at least 1 yoctoNEAR"
    )
}
