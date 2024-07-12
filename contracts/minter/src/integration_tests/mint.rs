#![cfg(test)]
use cosmwasm_std::{Addr, Coin, Uint128};

use cw_multi_test::Executor;

use crate::token::{Extension, QueryMsg as Cw721QueryMsg};
use crate::token::cw721::OwnerOfResponse;
use rstest::rstest;

use crate::contract::DENOM;
use crate::integration_tests::util::{
    create_cw721, create_whitelist_minter, init_whitelist_minter, mint_native, mock_app, query,
    NAME_PREFIX,
};
use crate::msg::{EnablePublicMintMsg, EnableWhitelistMintMsg, ExecuteMsg, MintMsg};
use crate::state::State;

// Only whitelisted users can mint during the whitelist
// period (e.g. reveal == false). After the whitelist
// period, public users can mint
#[rstest]
#[case(5)]
#[case(10)]
#[case(20)]
#[case(30)]
fn test_whitelist_permissions(#[case] whitelist_limit: u64) {
    let mut app = mock_app();

    // wlm_admin deploys and owns the wlm contract
    let wlm_admin = Addr::unchecked("whitelist_minter_owner");
    // wlm_artist owns the cw721
    let wlm_artist = Addr::unchecked("cw721_artist");
    // wlm_user mints whitelist nfts
    let wlm_user = Addr::unchecked("wlm_customer");
    // public_user mints non-whitelist nfts
    let public_user = Addr::unchecked("public_customer");

    // mint natve ARCH tokens to wlm_user
    mint_native(
        &mut app,
        wlm_user.to_string(),
        Uint128::from(10000000000000000000000_u128), // 100 ARCH as aarch
    );

    // mint natve ARCH tokens to public_user
    mint_native(
        &mut app,
        public_user.to_string(),
        Uint128::from(10000000000000000000000_u128), // 100 ARCH as aarch
    );

    // wlm_admin creates the wlm contract; cw721 address
    // is not yet properly initialized in wlm State
    let supply: u64 = 3333;
    let whitelist_allowance: u64 = whitelist_limit;
    let whitelist: Vec<Addr> = vec![wlm_user.clone()];
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
        public_whitelist_allowance: whitelist_allowance,
        price: Uint128::from(10000000000000000000_u128), // price
        name_prefix: NAME_PREFIX.to_string(),
    };
    let _res = init_whitelist_minter(&mut app, wlm_admin.clone(), wlm.clone(), config_update);

    // wlm_user must pay the correct minting price
    // (e.g. >= to state.price)
    let res = app.execute_contract(
        wlm_user.clone(),
        wlm.clone(),
        &ExecuteMsg::Mint(MintMsg {}),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(10000_u128), // not enough funds sent
        }],
    );
    assert!(res.is_err());

    // wlm_user must be allowed to mint during the
    // whitelist period
    let res = app.execute_contract(
        wlm_user.clone(),
        wlm.clone(),
        &ExecuteMsg::Mint(MintMsg {}),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(10000000000000000000_u128), // correct funds
        }],
    );
    assert!(res.is_ok());

    // wlm_user must own the token
    let token_id: String = (1).to_string();
    let owner_query: OwnerOfResponse = query(
        &mut app,
        nft.clone(),
        Cw721QueryMsg::<Extension>::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        },
    )
    .unwrap();
    assert_eq!(owner_query.owner, wlm_user.to_string());

    // during whitelist period, wlm_user can mint up
    // to whitelist_allowance nfts
    for _i in 1..whitelist_allowance {
        let res = app.execute_contract(
            wlm_user.clone(),
            wlm.clone(),
            &ExecuteMsg::Mint(MintMsg {}),
            &[Coin {
                denom: DENOM.to_string(),
                amount: Uint128::from(10000000000000000000_u128),
            }],
        );
        assert!(res.is_ok());
    }

    // wlm_user minting (whitelist_allowance + 1) must fail
    let res = app.execute_contract(
        wlm_user.clone(),
        wlm.clone(),
        &ExecuteMsg::Mint(MintMsg {}),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(10000000000000000000_u128),
        }],
    );
    assert!(res.is_err());

    // public_user must not be allowed to mint
    // during whitelist period
    let res = app.execute_contract(
        public_user.clone(),
        wlm.clone(),
        &ExecuteMsg::Mint(MintMsg {}),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(10000000000000000000_u128),
        }],
    );
    assert!(res.is_err());

    // wlm_admin enables public mint
    let _res = app.execute_contract(
        wlm_admin.clone(),
        wlm.clone(),
        &ExecuteMsg::EnablePublicMint(EnablePublicMintMsg {}),
        &[],
    );

    // public_user can mint during the public period
    let res = app.execute_contract(
        public_user.clone(),
        wlm.clone(),
        &ExecuteMsg::Mint(MintMsg {}),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(10000000000000000000_u128),
        }],
    );
    assert!(res.is_ok());

    // public_user must own the token
    let token_id: String = (whitelist_allowance + 1).to_string();
    let owner_query: OwnerOfResponse = query(
        &mut app,
        nft.clone(),
        Cw721QueryMsg::<Extension>::OwnerOf {
            token_id: token_id.clone(),
            include_expired: None,
        },
    )
    .unwrap();
    assert_eq!(owner_query.owner, public_user.to_string());
}

// Entire nft collection can be minted
#[test]
fn test_mint_all_nfts() {
    let mut app = mock_app();

    // wlm_admin deploys and owns the wlm contract
    let wlm_admin = Addr::unchecked("whitelist_minter_owner");
    // wlm_artist owns the cw721
    let wlm_artist = Addr::unchecked("cw721_artist");
    // wlm_user mints and reveals nfts, and pays the minting price
    let wlm_user = Addr::unchecked("some_customer");

    // wlm_admin creates the wlm contract; cw721 address
    // is not yet properly initialized in wlm State
    let supply: u64 = 3333;
    let whitelist_allowance: u64 = 5;
    let whitelist: Vec<Addr> = vec![wlm_admin.clone(), wlm_artist.clone(), wlm_user.clone()];
    let wlm = create_whitelist_minter(
        &mut app,
        wlm_admin.clone(),
        supply,
        whitelist_allowance,
        whitelist.clone(),
        wlm_artist.clone(),        // receipient of reserved NFTs
        Uint128::from(10000_u128), // minting price really cheap
    );

    // create the NFT contract with wlm as minter
    let nft = create_cw721(&mut app, &wlm);

    // mint natve ARCH tokens to wlm_user
    mint_native(
        &mut app,
        wlm_user.to_string(),
        Uint128::from(100000000000000000000_u128), // 100 ARCH as aarch
    );

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
        public_whitelist_allowance: whitelist_allowance,
        price: Uint128::from(10000_u128), // price
        name_prefix: NAME_PREFIX.to_string(),
    };
    init_whitelist_minter(&mut app, wlm_admin.clone(), wlm.clone(), config_update);

    // wlm_admin enables public mint
    app.execute_contract(
        wlm_admin.clone(),
        wlm.clone(),
        &ExecuteMsg::EnableNormalWhitelist(EnableWhitelistMintMsg {}),
        &[],
    ).unwrap();
    app.execute_contract(
        wlm_admin.clone(),
        wlm.clone(),
        &ExecuteMsg::EnablePublicMint(EnablePublicMintMsg {}),
        &[],
    ).unwrap();

    // wlm_user mints the remaining (supply - total_reserved) nfts
    let remaining: u64 = supply;
    let mut responses = vec![];
    for _i in 0..remaining {
        let res = app.execute_contract(
            wlm_user.clone(),
            wlm.clone(),
            &ExecuteMsg::Mint(MintMsg {}),
            &[Coin {
                denom: DENOM.to_string(),
                amount: Uint128::from(10000_u128),
            }],
        );
        assert!(res.is_ok());
        responses.push(res);
    }

    // minting more than total supply must return
    // a SoldOut error
    let res = app.execute_contract(
        wlm_user.clone(),
        wlm.clone(),
        &ExecuteMsg::Mint(MintMsg {}),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(10000_u128),
        }],
    );
    assert!(res.is_err());
}
