// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           31
// Async Callback:                       1
// Total number of exported functions:  34

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    liquid_staking
    (
        init => init
        upgrade => upgrade
        addLiquidity => add_liquidity
        removeLiquidity => remove_liquidity
        unbondTokens => unbond_tokens
        withdrawAll => withdraw_all
        claimRewards => claim_rewards
        recomputeTokenReserve => recompute_token_reserve
        delegateRewards => delegate_rewards
        getLsValueForPosition => get_ls_value_for_position
        registerLsToken => register_ls_token
        registerUnstakeToken => register_unstake_token
        setStateActive => set_state_active
        setStateInactive => set_state_inactive
        getState => state
        getLsTokenId => ls_token
        getLsSupply => ls_token_supply
        getVirtualEgldReserve => virtual_egld_reserve
        getRewardsReserve => rewards_reserve
        getTotalWithdrawnEgld => total_withdrawn_egld
        getUnstakeTokenId => unstake_token
        getUnstakeTokenSupply => unstake_token_supply
        whitelistDelegationContract => whitelist_delegation_contract
        changeDelegationContractAdmin => change_delegation_contract_admin
        changeDelegationContractParams => change_delegation_contract_params
        getDelegationStatus => get_delegation_status
        getDelegationContractStakedAmount => get_delegation_contract_staked_amount
        getDelegationContractUnstakedAmount => get_delegation_contract_unstaked_amount
        getDelegationContractUnbondedAmount => get_delegation_contract_unbonded_amount
        getDelegationAddressesList => delegation_addresses_list
        getAddressesToClaim => addresses_to_claim
        getDelegationClaimStatus => delegation_claim_status
        getDelegationContractData => delegation_contract_data
    )
}

multiversx_sc_wasm_adapter::async_callback! { liquid_staking }
