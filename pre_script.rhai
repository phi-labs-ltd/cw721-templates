let token_type = variable::get("token_type");

fn set_vars(cargo, lib, base_cargo, base_lib, save_snippet, execute_reveal, token_mint_params, token_status_query, test_reveal) {
    variable::set("cw721_cargo", cargo);
    variable::set("cw721_lib", lib);
    variable::set("cw721_base_cargo", base_cargo);
    variable::set("cw721_base_lib", base_lib);
    variable::set("minter_save_snippet", save_snippet);
    variable::set("execute_reveal", execute_reveal);
    variable::set("token_mint_params", token_mint_params);
    variable::set("token_status_query", token_status_query);
    variable::set("test_reveal", test_reveal);

    let crate_format = variable::get("project-name");
    crate_format.replace("-", "_");
    variable::set("crate_name", crate_format);
}

switch token_type {
    "non-updatable" => {
        file::delete("contracts/minter/src/integration_tests/reveal.rs");
        set_vars(
            "cw721 = \"0.18.0\"",
            "cw721",
            "cw721-base = { version = \"0.18.0\", features = [\"library\"] }",
            "cw721_base",
            "cw_ownable::initialize_owner(deps.storage, deps.api, Some(&minter.to_string()))?;",
            "return Err(ContractError::EntrypointDisabled {});",
            "{ token_id: token_id.clone().to_string(), owner: info.sender.clone().into(), token_uri: None, extension: None }",
            "return Err(ContractError::EntrypointDisabled {});",
            ""
        );
    }
    "updatable" => {
        set_vars(
            "cw721-updatable = \"1.0.0\"",
            "cw721_updatable",
            "cw721-base-updatable = { version = \"1.0.0\", features = [\"library\"] }",
            "cw721_base_updatable",
            "contract.minter.save(deps.storage, &minter)?;",
            `
                use crate::token::cw721_base::msg::UpdateMetadataMsg;
                // Expand to this if updatable
                let state = STATE.load(deps.storage)?;

                // Revealing must be enabled and minting concluded
                if !state.phase.is_reveal() {
                    return Err(ContractError::RevealDisabled {});
                }

                // Sender must own token to reveal it
                let query_msg: crate::token::QueryMsg<Extension> = Cw721QueryMsg::OwnerOf {
                    token_id: msg.token_id.clone(),
                    include_expired: None,
                };
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: state.cw721.clone().into(),
                    msg: to_json_binary(&query_msg).unwrap(),
                });
                let query_resp: OwnerOfResponse = deps.querier.query(&query_req)?;
                if query_resp.owner != info.sender {
                    return Err(ContractError::Unauthorized {});
                }
                // Token must not be already revealed
                let query_msg: crate::token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
                    token_id: msg.token_id.clone(),
                };
                let query_req = QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: state.cw721.clone().into(),
                    msg: to_json_binary(&query_msg).unwrap(),
                });
                let query_resp: NftInfoResponse<Extension> = deps.querier.query(&query_req)?;
                if query_resp.extension.is_some() {
                    return Err(ContractError::MetadataRevealed {
                        token_id: msg.token_id,
                    });
                }

                // TODO: implement reveal mechanics
                let next_reveal = Metadata::default();

                // Metadata update
                let update_msg: crate::token::ExecuteMsg =
                    Cw721ExecuteMsg::UpdateMetadata(UpdateMetadataMsg {
                        token_id: msg.token_id.clone(),
                        extension: Some(next_reveal),
                    });

                let update_resp: CosmosMsg = WasmMsg::Execute {
                    contract_addr: state.cw721.into(),
                    msg: to_json_binary(&update_msg)?,
                    funds: vec![],
                }
                .into();

                Ok(Response::new()
                    .add_attribute("action", "reveal")
                    .add_attribute("token_id", msg.token_id)
                    .add_message(update_resp))
            `,
            "(crate::token::cw721_base::MintMsg { token_id: token_id.clone().to_string(), owner: info.sender.clone().into(), token_uri: None, extension: None })",
            "",
            "mod reveal;",
        );
    }
}

file::delete("clean.sh");
file::delete("test.sh");