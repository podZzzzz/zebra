//! Consensus parameter tests for Zebra.

use super::*;
use NetworkUpgrade::*;

use std::collections::HashSet;

use zebra_chain::types::BlockHeight;
use zebra_chain::{Network, Network::*};

/// Check that the activation heights and network upgrades are unique.
#[test]
fn activation_bijective() {
    let mainnet_activations = NetworkUpgrade::activation_list(Mainnet);
    let mainnet_heights: HashSet<&BlockHeight> = mainnet_activations.keys().collect();
    assert_eq!(MAINNET_ACTIVATION_HEIGHTS.len(), mainnet_heights.len());

    let mainnet_nus: HashSet<&NetworkUpgrade> = mainnet_activations.values().collect();
    assert_eq!(MAINNET_ACTIVATION_HEIGHTS.len(), mainnet_nus.len());

    let testnet_activations = NetworkUpgrade::activation_list(Testnet);
    let testnet_heights: HashSet<&BlockHeight> = testnet_activations.keys().collect();
    assert_eq!(TESTNET_ACTIVATION_HEIGHTS.len(), testnet_heights.len());

    let testnet_nus: HashSet<&NetworkUpgrade> = testnet_activations.values().collect();
    assert_eq!(TESTNET_ACTIVATION_HEIGHTS.len(), testnet_nus.len());
}

#[test]
fn activation_extremes_mainnet() {
    activation_extremes(Mainnet)
}

#[test]
fn activation_extremes_testnet() {
    activation_extremes(Testnet)
}

/// Test the activation_list, activation_height, current, and next functions
/// for `network` with extreme values.
fn activation_extremes(network: Network) {
    // The first two upgrades are BeforeOverwinter and Overwinter
    assert_eq!(
        NetworkUpgrade::activation_list(network).get(&BlockHeight(0)),
        Some(&BeforeOverwinter)
    );
    assert_eq!(
        BeforeOverwinter.activation_height(network),
        Some(BlockHeight(0))
    );
    assert_eq!(
        NetworkUpgrade::current(network, BlockHeight(0)),
        BeforeOverwinter
    );
    assert_eq!(
        NetworkUpgrade::next(network, BlockHeight(0)),
        Some(Overwinter)
    );

    // We assume that the last upgrade we know about continues forever
    // (even if we suspect that won't be true)
    assert_ne!(
        NetworkUpgrade::activation_list(network).get(&BlockHeight::MAX),
        Some(&BeforeOverwinter)
    );
    assert_ne!(
        NetworkUpgrade::current(network, BlockHeight::MAX),
        BeforeOverwinter
    );
    assert_eq!(NetworkUpgrade::next(network, BlockHeight::MAX), None);
}

#[test]
fn activation_consistent_mainnet() {
    activation_consistent(Mainnet)
}

#[test]
fn activation_consistent_testnet() {
    activation_consistent(Testnet)
}

/// Check that the activation_height, current, and next functions are
/// consistent for `network`.
fn activation_consistent(network: Network) {
    let activation_list = NetworkUpgrade::activation_list(network);
    let network_upgrades: HashSet<&NetworkUpgrade> = activation_list.values().collect();

    for &network_upgrade in network_upgrades {
        let height = network_upgrade
            .activation_height(network)
            .expect("activations must have a height");
        assert_eq!(NetworkUpgrade::current(network, height), network_upgrade);
        // Network upgrades don't repeat
        assert_ne!(NetworkUpgrade::next(network, height), Some(network_upgrade));
        assert_ne!(
            NetworkUpgrade::next(network, BlockHeight(height.0 + 1)),
            Some(network_upgrade)
        );
        assert_ne!(
            NetworkUpgrade::next(network, BlockHeight::MAX),
            Some(network_upgrade)
        );
    }
}
