// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            5
// Async Callback (empty):               1
// Total number of exported functions:   7

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    delegation_mock
    (
        depositEGLD
        delegate
        unDelegate
        withdraw
        claimRewards
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
