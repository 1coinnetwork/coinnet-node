// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use node_runtime::constants::currency::*;
use node_runtime::Block;
use node_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig,
    CouncilConfig, DemocracyConfig, ElectionsConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig,
    SessionConfig, SessionKeys, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    TechnicalCommitteeConfig, MAX_NOMINATIONS,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

// /// 1COIN testnet generator
// pub fn coinnet_test_config() -> Result<ChainSpec, String> {
//     ChainSpec::from_json_bytes(&include_bytes!("../res/coinnet.json")[..])
// }

/// SWIVEL generator
pub fn swivel_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/swivel.json")[..])
}

// /// 1COIN main config
// pub fn coinnet_main_config() -> Result<ChainSpec, String> {
//     ChainSpec::from_json_bytes(&include_bytes!("../res/coinnet_main.json")[..])
// }

fn session_keys(
    grandpa: GrandpaId,
    babe: BabeId,
    im_online: ImOnlineId,
    authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
    }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
    // stash, controller, session-key
    // generated with secret:
    // for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
    // and
    // for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            // 5FsyzCpcUx9ZsdeRQFM6xjFqYemWb994rCanVxmawNRFRkSD
            hex!["a8c38854d898f1c8c3b8751a598cdbb5046b2be0458acecb7bcaa2aefe48e97f"].into(),
            // 5CVLvuo2jzX2bQvcSPZDrRwjPaJkWY73g4BCP5LRnr98CEVo
            hex!["12cab06435d27ad50ce77e5b1a5e65c6db6420654e4396afdecd96dbad85af07"].into(),
            // 5E4FinEQGCXvxysmoZzAiNSyUgTgizbJbXgnPCemXgiDgsg8
            hex!["58207c3b711c9939d0f43bac62b5174bafa800987f6185f7cc4da03466ca2c8f"]
                .unchecked_into(),
            // 5FxVCn66E51pcPW31SzRV5MtriqTttbFNXe67fFFUoKn4ERx
            hex!["ac32dddb72cc5aefe5763c4f14041ba09e48adc63e9c23167e78eb144e514744"]
                .unchecked_into(),
            // 5CttpVmzUEDLgVzyybpWmBcz7bXK6g9W7DXCfWQB999ie5s6
            hex!["24c0aec2c67b71f0e8c08345255d63544bdeb946569a61d61bcbdf4deda74846"]
                .unchecked_into(),
            // 5HbAL7sPkzQUNNcEYLbGef9dDVaz6r5Piq5kqNEKGhX42M6s
            hex!["f467a57b7254e52baccc378f856c7c70a114bb3dce52d6ccd08eb000e63cb45a"]
                .unchecked_into(),
        ),
        (
            // 5CaJka7STE3oDu1a7srV5A6eqXsu8Cq82piNSbDqdNRY8QPy
            hex!["16939556f51e5de8d9b67e5ffa39fb864c376ccf4d3b91fe3a2f2c46a18d5151"].into(),
            // 5EC9uDg71E9jTYqCRNiPnMiG13N6BjmB64sXHSHFawYG5d9z
            hex!["5e26db4867a15931ee06e1dfc2e21e564d76e6e113a8a7a4f7f580b01ad3d821"].into(),
            // 5E5MHMkre9yKDCrTGV22fFVTy4yhFeWSyBsnyZCxad1WiPi1
            hex!["58f673f80def417750141f0a436e050790c085ee0e5847f0218a9d88d4303d91"]
                .unchecked_into(),
            // 5GQqBSckpYPhgqKtC1fngWG41833yaxbumfxv65LuvyymKpA
            hex!["c04a7ee0fad92f36e8861b290ae21fb483aa8cd8128884dd2a07a806b502473e"]
                .unchecked_into(),
            // 5G18cF9LJ4ieimiBgfaeZySNBGv6Xa9wANH66hFBG8xxNLgm
            hex!["ae3745c46162de3e99e185a7016f45c4c6ead43758af83de2f4d1e43a6499d25"]
                .unchecked_into(),
            // 5Ff355W5nnWL92kby2AkPnS6QX6JeCaixmeXmh2ff9yd1Eqx
            hex!["9ee3b95c20406bc91b2a3a1a1e1906fff032a34fd45ec15d538e7da9b2d8193b"]
                .unchecked_into(),
        ),
        (
            // 5Dr2MUWQ9ELYytRAaKjnKjcTYDf5img4TywkJy3Cyg9NYMzC
            hex!["4ecc8abddd82e8df76006a016c28c5efec7efa97c390d3b268ddc958d24f1345"].into(),
            // 5Hpnos8MQAUmcSKDY79ZPG95yigEhpR8DXqjXdGaL3ViwcJJ
            hex!["fecca242d93357cfb7c09b41c1e4778373109bcf36f1df9b6fb4052085c7bd27"].into(),
            // 5Gn3eszzNNyL3z1Di3fBw52qZiT4dUyqFhEXkWNXNXwzRi1B
            hex!["d078a31e8340cb7d4776e85886a54dbd6fac62e00dbab71a8cb2549117aaf030"]
                .unchecked_into(),
            // 5FZENnmsv12Ay8AMbqhYWYi3UMTNzMmZjZBqbLc26Vxzm51o
            hex!["9a764f14d06d0b1315500a2eeb6661409e9ead1bfe98c1529e95fbcb03194d41"]
                .unchecked_into(),
            // 5D7cwQziPKxHggd5ZrvkqtgDGCPcd6P31KY9zzcvNe7qc5pS
            hex!["2e7569e242cc37b0cd36988e37d556347adef3e7f5748cecab4b4e94ed408a06"]
                .unchecked_into(),
            // 5H9UD5uhN1eBs9qnnx5bunMed27CaxTZVxvATihMaR6hMTYf
            hex!["e0cf7378e7dd916f323b2129117b75a50dcc3441d7893e584657112916e1cc7b"]
                .unchecked_into(),
        ),
    ];

    let root_key: AccountId = hex![
        // 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
        "e4d1672364a0a7bf32508b2cd6e200e4fe8c13b8f763cc7ac258ad66c6299c11"
    ]
    .into();

    let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

    testnet_genesis(
        initial_authorities,
        vec![],
        root_key,
        Some(endowed_accounts),
        false,
    )
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Swivel",
        "swivel",
        ChainType::Live,
        staging_testnet_config_genesis,
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Staging telemetry url is valid; qed"),
        ),
        None,
        None,
        Default::default(),
    )
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    enable_println: bool,
) -> GenesisConfig {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(&x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
        .chain(initial_nominators.iter().map(|x| {
            use rand::{seq::SliceRandom, Rng};
            let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
            let count = rng.gen::<usize>() % limit;
            let nominations = initial_authorities
                .as_slice()
                .choose_multiple(&mut rng, count)
                .into_iter()
                .map(|choice| choice.0.clone())
                .collect::<Vec<_>>();
            (
                x.clone(),
                x.clone(),
                STASH,
                StakerStatus::Nominator(nominations),
            )
        }))
        .collect::<Vec<_>>();

    let num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
    const STASH: Balance = ENDOWMENT / 1000;

    GenesisConfig {
        frame_system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| (x, ENDOWMENT))
                .collect(),
        },
        pallet_indices: IndicesConfig { indices: vec![] },
        pallet_session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        pallet_staking: StakingConfig {
            validator_count: initial_authorities.len() as u32 * 2,
            minimum_validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            stakers,
            ..Default::default()
        },
        pallet_democracy: DemocracyConfig::default(),
        pallet_elections_phragmen: ElectionsConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .map(|member| (member, STASH))
                .collect(),
        },
        pallet_collective_Instance1: CouncilConfig::default(),
        pallet_collective_Instance2: TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        },
        pallet_contracts: ContractsConfig {
            // println should only be enabled on development chains
            current_schedule: pallet_contracts::Schedule::default().enable_println(enable_println),
        },
        pallet_sudo: SudoConfig { key: root_key },
        pallet_babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(node_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        pallet_im_online: ImOnlineConfig { keys: vec![] },
        pallet_authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        pallet_grandpa: GrandpaConfig {
            authorities: vec![],
        },
        pallet_membership_Instance1: Default::default(),
        pallet_treasury: Default::default(),
        pallet_vesting: Default::default(),
    }
}

fn development_config_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        true,
    )
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
        false,
    )
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::service::{new_full_base, new_light_base, NewFullBase};
    use sc_service_test;
    use sp_runtime::BuildStorage;

    fn local_testnet_genesis_instant_single() -> GenesisConfig {
        testnet_genesis(
            vec![authority_keys_from_seed("Alice")],
            vec![],
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            None,
            false,
        )
    }

    /// Local testnet config (single validator - Alice)
    pub fn integration_test_config_with_single_authority() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis_instant_single,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    /// Local testnet config (multivalidator Alice + Bob)
    pub fn integration_test_config_with_two_authorities() -> ChainSpec {
        ChainSpec::from_genesis(
            "Integration Test",
            "test",
            ChainType::Development,
            local_testnet_genesis,
            vec![],
            None,
            None,
            None,
            Default::default(),
        )
    }

    #[test]
    #[ignore]
    fn test_connectivity() {
        sc_service_test::connectivity(
            integration_test_config_with_two_authorities(),
            |config| {
                let NewFullBase {
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                    ..
                } = new_full_base(config, |_, _| ())?;
                Ok(sc_service_test::TestNetComponents::new(
                    task_manager,
                    client,
                    network,
                    transaction_pool,
                ))
            },
            |config| {
                let (keep_alive, _, client, network, transaction_pool) = new_light_base(config)?;
                Ok(sc_service_test::TestNetComponents::new(
                    keep_alive,
                    client,
                    network,
                    transaction_pool,
                ))
            },
        );
    }

    #[test]
    fn test_create_development_chain_spec() {
        development_config().build_storage().unwrap();
    }

    #[test]
    fn test_create_local_testnet_chain_spec() {
        local_testnet_config().build_storage().unwrap();
    }

    #[test]
    fn test_staging_test_net_chain_spec() {
        staging_testnet_config().build_storage().unwrap();
    }
}
