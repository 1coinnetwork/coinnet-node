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
    SessionConfig, SessionKeys, SocietyConfig, StakerStatus, StakingConfig, SudoConfig,
    SystemConfig, TechnicalCommitteeConfig, MAX_NOMINATIONS,
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

/// 1COIN testnet generator
pub fn coinnet_test_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/coinnet.json")[..])
}

/// 1COIN main config
pub fn coinnet_main_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/coinnet_main.json")[..])
}

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
            // 5DZgiavMzv3NonUEG7CUPTReg25aLpaTDWdJcpnAwMNT3BUC
            hex!["425683c5e9814ef3e429d1a80f5728d2bb7809408751f011f1982aaf11706646"].into(),
            // 5F1umJsBGAaFmLvme1c5UqEHvN6WKEovLgud7fHab9gGcGE2
            hex!["82930c74ec05211f19372558ff009213c7b19f2a3b860962f31d4abaffd8f81a"].into(),
            // 5DKD6rZUbt5tFktfwSwjzrDnig4wQf6pQQfF29ScoHJuPEir
            hex!["374c1d7da6467bca73a126279f0f8e3836e7ae56a3f1fe4ae7dc1d6cdcb97c38"]
                .unchecked_into(),
            // 5GYRmtFRP38xfDGot11QGqTRkfGD5Pf8CZRN3E3HdajV4Pc7
            hex!["c615aaa9ac54dd543f3aed5ef80f29d0e2216a432f2a7df887c15140a42a1575"]
                .unchecked_into(),
            // 5HGtQX4JJW3YtWmNp9hRYDemJRy7dW7tWe2LVwzEiqi9YELC
            hex!["e6779faaa6f2a0f66a708700b63b946edf66655c61b218c4a0863b52af24ae09"]
                .unchecked_into(),
            // 5HgmHLHrwJbgSN8PagC1kvZavm26b9LpG17WFL1Vd7WwvuFM
            hex!["f8ad88419580b55c0877adf9e67633573611f77b090e46ed0ea96952794c815e"]
                .unchecked_into(),
        ),
        (
            // 5Gzjn3qt5uhMBMAGbne4sjiE644TsAwtas84bVU2GvzyMgTc
            hex!["da26a67157b30108c941c31b46af635f878b172c12cac69e58fcd804dd5bf641"].into(),
            // 5Gq98dH63gmUWQanyCVicUFMbe37SsbxBWd88BpwVVHX4rNm
            hex!["d2d4d0b5e7e55af834ea3ba3fba92e83cb798577083ca4b7ee01e1591c473d5d"].into(),
            // 5G6nerT3DQ1g5A5rku8H5xETDu5QRh1UYGyvjdtbx1MYZvbe
            hex!["b2879203306f223216579c3d66429b0f915b307f3acaeac95de935adcbff5ddc"]
                .unchecked_into(),
            // 5EUGbWmBDe6A4mDkgZ2xbL4MBRD2kWXHiu6frcL2MeY8fJ7P
            hex!["6a7151ccaa67b1ae76b38e901b435657e129761b2b55cec5d804778c873d5970"]
                .unchecked_into(),
            // 5DFksnt5uL8YYa2gnVsrCspp5Tf6fWsfHU9Q4LfV5ZydQdD4
            hex!["34aa18d41765d438de24bd118f55f12dacd9ab31d139b8dad154e9581e9a5352"]
                .unchecked_into(),
            // 5Gmh6NBUBXFDZCGJsqnQWDb9KXZ6Tvj37DhTWXhp8R6J9xbr
            hex!["d0336c901f187212502d14264cbfa448aa97aa19dbf6f16a75627b4fcffabd52"]
                .unchecked_into(),
        ),
        (
            // 5GLCQWnzb4b1WLnEHNoMVTziQ2M4cCkzV5p8DaywMvQHsb3i
            hex!["bcc1b837623631376e589c099c87dc598b0dce97fa428bae5411064dbee91a34"].into(),
            // 5E4bj6Kd7MfTJwdy1uYSkgc5p6qVSHJUhf97FuLN1wC8oeDL
            hex!["5863d452a28c5b2db6d35bc6ec0bcf12debfa9fc25cf343044033c5fb0f88f6a"].into(),
            // 5DippvxRqcJ3LgpDAtgNChVH7cW3DRRjMSy7jBw5mjD3547S
            hex!["494f05a61a29c4d6f5bdc302ac4eedcba6abb097949bbf20f421248d1378ab06"]
                .unchecked_into(),
            // 5Dvz95Z1FfWwgRzNRSgTAN1MQxhD1SKEtjRxyWehGjrPnPe8
            hex!["52955122a092772adc39930a4b77a649a5dd2ad86ae5a84242645290b812bc1b"]
                .unchecked_into(),
            // 5DP4nPUNKpUgiU5vrEyFdtrh6oQcJCuUGdEpHeiK4rCwF7Az
            hex!["3a3d18375fd0a6eddc7991ebf3ddf3c0140db55ec5b3cb7e99fc8bd95359d917"]
                .unchecked_into(),
            // 5EjvQ5CM6pyUSR9b2sTrZTiMH9KUa5HCkEKMSt6zyzK3DNKp
            hex!["76614187c2122ac8b70c8fc40431916cfb33379f1db6af87b5ea4c2dc28eaf0e"]
                .unchecked_into(),
        ),
    ];

    let root_key: AccountId = hex![
        // 5Fk6QsYKvDXxdXumGdHnNQ7V7FziREy6qn8WjDLEWF8WsbU3
        "326533ecd0d499ed73c1ecba838f52e95f238a4f212458aae39462f89c447040"
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
        "Staging Testnet",
        "staging_testnet",
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
        pallet_society: SocietyConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            pot: 0,
            max_members: 999,
        },
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
