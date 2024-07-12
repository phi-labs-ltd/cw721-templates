#![cfg(test)]
use cosmwasm_std::{Addr, Uint128};

use crate::token::{Extension, QueryMsg as Cw721QueryMsg};
use crate::token::cw721::{NumTokensResponse, TokensResponse};

use crate::integration_tests::util::{
    create_cw721, create_whitelist_minter, init_whitelist_minter, mock_app, query, NAME_PREFIX,
};
use crate::state::State;

#[test]
fn test_initialize() {
    let mut app = mock_app();

    // wlm_admin deploys and owns the wlm contract
    let wlm_admin = Addr::unchecked("whitelist_minter_owner");
    // wlm_artist owns the cw721
    let wlm_artist = Addr::unchecked("cw721_artist");

    // wlm_admin creates the wlm contract; cw721 address
    // is not yet properly initialized in wlm State
    let supply: u64 = 3333;
    let whitelist_allowance: u64 = 5;
    let whitelist: Vec<Addr> = vec![wlm_admin.clone(), wlm_artist.clone()];
    let wlm = create_whitelist_minter(
        &mut app,
        wlm_admin.clone(),
        supply,
        whitelist_allowance,
        whitelist.clone(),
        wlm_artist.clone(),                       // receipient of reserved NFTs
        Uint128::from(10000000000000000000_u128), // minting price of 10 ARCH (as aarch)
    );

    // create the NFT contract with wlm as minter
    let nft = create_cw721(&mut app, &wlm);

    // wlm_admin updates the wlm contract with
    // the correct cw721 address to be used for
    // minting and revealing
    let config_update = State {
        owner: wlm_admin.clone(),
        cw721: nft.clone(),
        artist: wlm_artist.clone(),
        supply,
        phase: Default::default(),
        private_whitelist_allowance: whitelist_allowance,
        price: Uint128::from(10000000000000000000_u128), // price
        name_prefix: NAME_PREFIX.to_string(),
        public_whitelist_allowance: whitelist_allowance,
    };
    let _res = init_whitelist_minter(&mut app, wlm_admin.clone(), wlm.clone(), config_update);
}
