#![no_std]

//! Planter Registry Contract — Closes #475
//!
//! On-chain registry, reputation scoring, and slash mechanism for planters
//! (the farmers who actually plant the trees funded via the escrow contracts).
//!
//! # Operations
//!
//! | Function            | Auth      | Effect                                                     |
//! |---------------------|-----------|------------------------------------------------------------|
//! | `register_planter`  | self      | Onboard a planter with `land_doc_hash` + `region_geohash`. |
//! | `rate_planter`      | sponsor   | 1 – 5 star rating; appended to planter's reputation.       |
//! | `slash_planter`     | admin     | Penalty: bumps `slash_count` and toggles `available=false`.|
//! | `reset_slash`       | admin     | Clears all slashes and marks the planter available again.  |
//! | `is_registered`     | public    | Registry membership check.                                 |
//! | `is_available`      | public    | Opt-in availability.                                       |
//! | `get_profile`       | public    | Current planter profile.                                   |
//! | `get_reputation`    | public    | Aggregated score (0 – 100, 5 stars × 20 = 100).             |
//! | `get_sponsor_rating`| public    | Single sponsor's most recent rating for the planter, or None.|
//!
//! # Storage
//!
//!   - Instance: `ADMIN` (governance for `slash_planter` / `reset_slash`).
//!   - Persistent: `Planter(Address) -> PlanterProfile`.
//!   - Persistent: `Rating(sponsor, planter) -> u32` (last rating).
//!   - Persistent: `Reputation(Address) -> PlanterReputation`.
//!
//! # Events
//!
//!   - `PlanterReg(planter)`            — registration.
//!   - `PlanterRated(planter)`          — `(sponsor, rating, average_rating)`.
//!   - `PlanterSlashed(planter)`        — `(new_slash_count, available=false)`.
//!   - `SlashReset(planter)`            — `(cleared_count)`.

use harvesta_errors::HarvestaError;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, symbol_short, Address, BytesN, Env,
    IntoVal, String,
};

// ── Types ─────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PlanterProfile {
    pub wallet_address: Address,
    pub land_doc_hash: BytesN<32>,
    pub region_geohash: String,
    pub registered_at: u64,
    pub available: bool,
    pub slash_count: u32,
}

/// Aggregated reputation for a planter.
///
/// `average_rating` is scaled 0 – 100 (so a single 5-star rating reads
/// as 100 and a single 1-star reads as 20). This avoids float math.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PlanterReputation {
    pub planter: Address,
    pub total_ratings: u32,
    pub sum_ratings: u128,
    pub average_rating: u32,
}

// ── Storage keys ──────────────────────────────────────────────────────────────

fn admin_key() -> soroban_sdk::Symbol {
    symbol_short!("ADMIN")
}

fn planter_key(env: &Env, planter: &Address) -> soroban_sdk::Val {
    (symbol_short!("PLANT"), planter.clone()).into_val(env)
}

fn reputation_key(env: &Env, planter: &Address) -> soroban_sdk::Val {
    (symbol_short!("REP"), planter.clone()).into_val(env)
}

fn rating_key(env: &Env, sponsor: &Address, planter: &Address) -> soroban_sdk::Val {
    (symbol_short!("RT"), sponsor.clone(), planter.clone()).into_val(env)
}

// ── Constants ────────────────────────────────────────────────────────────────

/// Minimum legal star rating.
const MIN_RATING: u32 = 1;
/// Maximum legal star rating.
const MAX_RATING: u32 = 5;
/// Scaling factor so that `average_rating` fits in `u32` (max 5 stars × 20 = 100).
const RATING_SCALE: u128 = 20;

/// Northern-Nigeria 2-char geohash prefixes (mirrors `farmer-registry`'s
/// validation so the same off-chain geocoder can be used).
const VALID_REGIONS: [&str; 9] = [
    "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8",
];

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct PlanterRegistry;

#[contractimpl]
impl PlanterRegistry {
    /// One-time setup. `admin` is the only address that may slash or
    /// reset slashes.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&admin_key()) {
            panic_with_error!(&env, HarvestaError::AlreadyInitialized);
        }
        env.storage().instance().set(&admin_key(), &admin);
    }

    // ── register ─────────────────────────────────────────────────────────────

    /// Onboard a planter. `wallet_address.require_auth()` ensures the
    /// planter themselves (the key holder) consented to onboarding.
    /// Returns the freshly-registered profile.
    pub fn register_planter(
        env: Env,
        wallet_address: Address,
        land_doc_hash: BytesN<32>,
        region_geohash: String,
    ) -> PlanterProfile {
        wallet_address.require_auth();

        Self::assert_valid_region(&env, &region_geohash);

        let key = planter_key(&env, &wallet_address);
        if env.storage().persistent().has(&key) {
            panic_with_error!(&env, HarvestaError::FarmerAlreadyRegistered);
        }

        let profile = PlanterProfile {
            wallet_address: wallet_address.clone(),
            land_doc_hash,
            region_geohash,
            registered_at: env.ledger().timestamp(),
            available: true,
            slash_count: 0,
        };

        env.storage().persistent().set(&key, &profile);

        // Initialise empty reputation record so reads before any rating are deterministic.
        env.storage().persistent().set(
            &reputation_key(&env, &wallet_address),
            &PlanterReputation {
                planter: wallet_address.clone(),
                total_ratings: 0,
                sum_ratings: 0,
                average_rating: 0,
            },
        );

        env.events().publish(
            (symbol_short!("PlanterReg"), wallet_address.clone()),
            profile.clone(),
        );

        profile
    }

    // ── rate (score) ─────────────────────────────────────────────────────────

    /// Sponsor rates a planter 1 – 5 stars. One rating per
    /// `(sponsor, planter)` pair: re-rating overwrites the previous value
    /// and updates the aggregated reputation atomically.
    ///
    /// Sponsor must be authenticated. The planter does not need to be
    /// registered for a rating to land — but unregistered planters will
    /// not appear in `get_profile` lookups; the rating data is still kept.
    pub fn rate_planter(env: Env, sponsor: Address, planter: Address, rating: u32) {
        sponsor.require_auth();

        if rating < MIN_RATING || rating > MAX_RATING {
            panic!("rating must be between 1 and 5");
        }

        let rkey = rating_key(&env, &sponsor, &planter);
        let previous: Option<u32> = env.storage().persistent().get(&rkey);

        let rep_key = reputation_key(&env, &planter);
        let mut rep: PlanterReputation = env
            .storage()
            .persistent()
            .get(&rep_key)
            .unwrap_or(PlanterReputation {
                planter: planter.clone(),
                total_ratings: 0,
                sum_ratings: 0,
                average_rating: 0,
            });

        // Roll back the previous rating, if any, before applying the new one.
        if let Some(prev) = previous {
            rep.total_ratings = rep
                .total_ratings
                .checked_sub(1)
                .expect("rating count underflow");
            rep.sum_ratings = rep
                .sum_ratings
                .checked_sub(prev as u128)
                .expect("rating sum underflow");
        }

        rep.total_ratings = rep
            .total_ratings
            .checked_add(1)
            .expect("rating count overflow");
        rep.sum_ratings = rep
            .sum_ratings
            .checked_add(rating as u128)
            .expect("rating sum overflow");
        rep.average_rating = Self::compute_average(&rep);

        env.storage().persistent().set(&rkey, &rating);
        env.storage().persistent().set(&rep_key, &rep);

        env.events().publish(
            (symbol_short!("PlanterRated"), planter.clone()),
            (sponsor, rating, rep.average_rating),
        );
    }

    // ── slash ────────────────────────────────────────────────────────────────

    /// Admin slashes a planter. Each call bumps `slash_count` and toggles
    /// `available = false`. Rate-planter is unaffected (existing ratings
    /// remain in the reputation record) — the slash is a *gating*
    /// penalty, not a *forgetting* one.
    pub fn slash_planter(env: Env, admin: Address, planter: Address) {
        Self::require_admin(&env, &admin);

        let key = planter_key(&env, &planter);
        let mut profile: PlanterProfile = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, HarvestaError::FarmerNotRegistered));

        profile.slash_count = profile
            .slash_count
            .checked_add(1)
            .expect("slash count overflow");
        profile.available = false;

        env.storage().persistent().set(&key, &profile);

        env.events().publish(
            (symbol_short!("PlanterSlashed"), planter.clone()),
            (profile.slash_count, profile.available),
        );
    }

    /// Admin clears all slashes against a planter and re-marks them
    /// available. Existing ratings are preserved.
    pub fn reset_slash(env: Env, admin: Address, planter: Address) {
        Self::require_admin(&env, &admin);

        let key = planter_key(&env, &planter);
        let mut profile: PlanterProfile = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, HarvestaError::FarmerNotRegistered));

        let cleared = profile.slash_count;
        profile.slash_count = 0;
        profile.available = true;

        env.storage().persistent().set(&key, &profile);

        env.events()
            .publish((symbol_short!("SlashReset"), planter.clone()), cleared);
    }

    // ── queries ──────────────────────────────────────────────────────────────

    pub fn is_registered(env: Env, planter: Address) -> bool {
        env.storage()
            .persistent()
            .has(&planter_key(&env, &planter))
    }

    pub fn is_available(env: Env, planter: Address) -> bool {
        env.storage()
            .persistent()
            .get::<_, PlanterProfile>(&planter_key(&env, &planter))
            .map(|p| p.available)
            .unwrap_or(false)
    }

    pub fn get_profile(env: Env, planter: Address) -> Option<PlanterProfile> {
        env.storage()
            .persistent()
            .get(&planter_key(&env, &planter))
    }

    pub fn get_reputation(env: Env, planter: Address) -> Option<PlanterReputation> {
        env.storage()
            .persistent()
            .get(&reputation_key(&env, &planter))
    }

    pub fn get_sponsor_rating(env: Env, sponsor: Address, planter: Address) -> Option<u32> {
        env.storage()
            .persistent()
            .get(&rating_key(&env, &sponsor, &planter))
    }

    // ── internal ──────────────────────────────────────────────────────────────

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&admin_key())
            .unwrap_or_else(|| panic_with_error!(env, HarvestaError::NotInitialized));
        if *caller != admin {
            panic_with_error!(env, HarvestaError::Unauthorized);
        }
        caller.require_auth();
    }

    fn assert_valid_region(env: &Env, region: &String) {
        for prefix in VALID_REGIONS {
            if *region == String::from_str(env, prefix) {
                return;
            }
        }
        panic_with_error!(env, HarvestaError::InvalidRegion);
    }

    fn compute_average(rep: &PlanterReputation) -> u32 {
        if rep.total_ratings == 0 {
            return 0;
        }
        ((rep.sum_ratings * RATING_SCALE) / rep.total_ratings as u128) as u32
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

    fn setup() -> (Env, Address, PlanterRegistryClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, PlanterRegistry);
        let client = PlanterRegistryClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        (env, admin, client)
    }

    fn land_hash(env: &Env, seed: u8) -> BytesN<32> {
        BytesN::from_array(env, &[seed; 32])
    }

    // ── initialise ───────────────────────────────────────────────────────────

    #[test]
    fn test_initialize_panics_on_double_init_via_helper() {
        // This test is a placeholder that exercises the helper wiring
        // (a registered admin means `slash_planter` will auth-path).
        // The actual double-init panic is asserted by the test below.
        let (env, admin, client) = setup();
        let imposter = Address::generate(&env);
        // Calling `slash_planter` as a non-admin should panic with `Unauthorized` (#3).
        // We use this contract-specific behaviour to indirectly confirm the admin is set.
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.slash_planter(&imposter, &Address::generate(&env));
        }));
        assert!(res.is_err(), "non-admin slash must panic");
        let _ = admin;
        let _ = env;
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_double_initialize_rejected() {
        let (_env, _admin, client) = setup();
        let imposter = Address::generate(&_env);
        client.initialize(&imposter);
    }

    // ── register_planter ─────────────────────────────────────────────────────

    #[test]
    fn test_register_planter_stores_profile_and_emits_event() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);

        let profile = client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        assert_eq!(profile.wallet_address, planter);
        assert_eq!(profile.land_doc_hash, land_hash(&env, 1));
        assert_eq!(profile.region_geohash, String::from_str(&env, "s1"));
        assert!(profile.available);
        assert_eq!(profile.slash_count, 0);

        let stored = client.get_profile(&planter).unwrap();
        assert_eq!(stored, profile);
        assert!(client.is_registered(&planter));
        assert!(client.is_available(&planter));
    }

    #[test]
    fn test_register_initialises_empty_reputation() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.total_ratings, 0);
        assert_eq!(rep.sum_ratings, 0);
        assert_eq!(rep.average_rating, 0);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #35)")]
    fn test_register_duplicate_rejected() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.register_planter(
            &planter,
            &land_hash(&env, 2),
            &String::from_str(&env, "s2"),
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #37)")]
    fn test_register_invalid_region_rejected() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "e7"), // not a Northern Nigeria prefix
        );
    }

    #[test]
    fn test_register_all_valid_regions() {
        let (env, _, client) = setup();
        for (i, prefix) in VALID_REGIONS.iter().enumerate() {
            let planter = Address::generate(&env);
            client.register_planter(
                &planter,
                &land_hash(&env, i as u8),
                &String::from_str(&env, prefix),
            );
            assert!(
                client.is_registered(&planter),
                "registration must succeed for {prefix}"
            );
        }
    }

    #[test]
    fn test_register_two_planters_are_independent() {
        let (env, _, client) = setup();
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        client.register_planter(&a, &land_hash(&env, 1), &String::from_str(&env, "s1"));
        client.register_planter(&b, &land_hash(&env, 2), &String::from_str(&env, "s2"));
        let ra = client.get_reputation(&a).unwrap();
        let rb = client.get_reputation(&b).unwrap();
        assert_eq!(ra.total_ratings, 0);
        assert_eq!(rb.total_ratings, 0);
    }

    // ── rate_planter (score) ────────────────────────────────────────────────

    #[test]
    fn test_rate_first_star_aggregates_correctly() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.rate_planter(&sponsor, &planter, &5u32);

        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.total_ratings, 1);
        assert_eq!(rep.sum_ratings, 5);
        // 5 × 20 / 1 = 100
        assert_eq!(rep.average_rating, 100);
        assert_eq!(client.get_sponsor_rating(&sponsor, &planter).unwrap(), 5);
    }

    #[test]
    fn test_rate_multiple_sponsors_accumulate() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        // 5, 4, 3, 2, 1 from five sponsors → average = 3, average_rating = 60.
        for star in 1u32..=5u32 {
            let s = Address::generate(&env);
            let rating = 6 - star; // 5,4,3,2,1
            client.rate_planter(&s, &planter, &rating);
        }

        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.total_ratings, 5);
        assert_eq!(rep.sum_ratings, 15);
        // 15 × 20 / 5 = 60
        assert_eq!(rep.average_rating, 60);
    }

    #[test]
    fn test_re_rating_overwrites_previous_rating() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        client.rate_planter(&sponsor, &planter, &1u32);
        client.rate_planter(&sponsor, &planter, &5u32);

        let rep = client.get_reputation(&planter).unwrap();
        // count stays at 1 because the second rating overwrote the first.
        assert_eq!(rep.total_ratings, 1);
        assert_eq!(rep.sum_ratings, 5);
        assert_eq!(rep.average_rating, 100);
        assert_eq!(client.get_sponsor_rating(&sponsor, &planter).unwrap(), 5);
    }

    #[test]
    fn test_one_sponsor_per_planter_invariant() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        // 10 re-ratings from the same sponsor — count must stay at 1.
        for rating in 1u32..=5u32 {
            client.rate_planter(&sponsor, &planter, &rating);
        }
        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.total_ratings, 1);
    }

    #[test]
    #[should_panic]
    fn test_rate_zero_rejected() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.rate_planter(&sponsor, &planter, &0u32);
    }

    #[test]
    #[should_panic]
    fn test_rate_six_rejected() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.rate_planter(&sponsor, &planter, &6u32);
    }

    #[test]
    fn test_rating_unregistered_planter_still_records_reputation() {
        // A sponsor may rate an unregistered planter — the contract keeps
        // reputation data even without a profile. is_available returns false
        // for un-registered addresses; get_profile returns None.
        let (env, _, client) = setup();
        let phantom_planter = Address::generate(&env);
        let sponsor = Address::generate(&env);

        client.rate_planter(&sponsor, &phantom_planter, &4u32);

        let rep = client.get_reputation(&phantom_planter).unwrap();
        assert_eq!(rep.total_ratings, 1);
        assert_eq!(rep.average_rating, 80); // 4 × 20 / 1
        assert!(client.get_profile(&phantom_planter).is_none());
        assert!(!client.is_available(&phantom_planter));
    }

    #[test]
    fn test_average_calculation_two_ratings() {
        let (env, _, client) = setup();
        let planter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        let s1 = Address::generate(&env);
        let s2 = Address::generate(&env);
        client.rate_planter(&s1, &planter, &3u32);
        client.rate_planter(&s2, &planter, &5u32);
        // (3 + 5) * 20 / 2 = 80
        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.average_rating, 80);
    }

    // ── slash_planter ────────────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_slash_unauthorised_caller_rejected() {
        let (env, _admin, client) = setup();
        let planter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        let imposter = Address::generate(&env);
        client.slash_planter(&imposter, &planter);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #36)")]
    fn test_slash_unregistered_planter_rejected() {
        let (env, admin, client) = setup();
        let phantom_planter = Address::generate(&env);
        client.slash_planter(&admin, &phantom_planter);
    }

    #[test]
    fn test_slash_by_admin_increments_count_and_unavails() {
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        assert!(client.is_available(&planter));

        client.slash_planter(&admin, &planter);

        let updated = client.get_profile(&planter).unwrap();
        assert_eq!(updated.slash_count, 1);
        assert!(!updated.available);
        assert!(!client.is_available(&planter));
    }

    #[test]
    fn test_multiple_slashes_accumulate_count() {
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        for expected in 1u32..=5u32 {
            client.slash_planter(&admin, &planter);
            assert_eq!(client.get_profile(&planter).unwrap().slash_count, expected);
        }
    }

    #[test]
    fn test_slash_does_not_erase_reputation() {
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.rate_planter(&sponsor, &planter, &5u32);

        let rep_before = client.get_reputation(&planter).unwrap();
        assert_eq!(rep_before.average_rating, 100);

        client.slash_planter(&admin, &planter);

        let rep_after = client.get_reputation(&planter).unwrap();
        assert_eq!(rep_after, rep_before, "slash must not affect reputation");
        assert!(!client.is_available(&planter));
    }

    // ── reset_slash ──────────────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_reset_slash_unauthorised_rejected() {
        let (env, _admin, client) = setup();
        let planter = Address::generate(&env);
        let imposter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.reset_slash(&imposter, &planter);
    }

    #[test]
    fn test_reset_slash_clears_count_and_restores_availability() {
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.slash_planter(&admin, &planter);
        client.slash_planter(&admin, &planter);
        client.slash_planter(&admin, &planter);
        assert_eq!(client.get_profile(&planter).unwrap().slash_count, 3);
        assert!(!client.is_available(&planter));

        client.reset_slash(&admin, &planter);

        let after = client.get_profile(&planter).unwrap();
        assert_eq!(after.slash_count, 0);
        assert!(after.available);
        assert!(client.is_available(&planter));
    }

    #[test]
    fn test_reset_slash_after_zero_slashes_is_no_op() {
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );

        client.reset_slash(&admin, &planter);
        let after = client.get_profile(&planter).unwrap();
        assert_eq!(after.slash_count, 0);
        assert!(after.available);
    }

    // ── integration: full register → rate → slash → reset → rate cycle ───────

    #[test]
    fn test_full_lifecycle_register_rate_slash_reset_rate() {
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        let sponsor_a = Address::generate(&env);
        let sponsor_b = Address::generate(&env);

        // Register.
        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s3"),
        );
        assert!(client.is_registered(&planter));
        assert!(client.is_available(&planter));
        assert_eq!(client.get_reputation(&planter).unwrap().total_ratings, 0);

        // 3 sponsors rate 5, 4, 3 → average (5+4+3) * 20 / 3 = 80.
        client.rate_planter(&sponsor_a, &planter, &5u32);
        client.rate_planter(&sponsor_b, &planter, &4u32);
        let s3 = Address::generate(&env);
        client.rate_planter(&s3, &planter, &3u32);

        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.total_ratings, 3);
        assert_eq!(rep.sum_ratings, 12);
        assert_eq!(rep.average_rating, 80);

        // Admin slashes twice.
        client.slash_planter(&admin, &planter);
        client.slash_planter(&admin, &planter);
        assert!(!client.is_available(&planter));
        assert_eq!(client.get_profile(&planter).unwrap().slash_count, 2);

        // Reputation survives the slash.
        let rep_after_slash = client.get_reputation(&planter).unwrap();
        assert_eq!(rep_after_slash.average_rating, 80);

        // Admin resets.
        client.reset_slash(&admin, &planter);
        assert!(client.is_available(&planter));
        assert_eq!(client.get_profile(&planter).unwrap().slash_count, 0);

        // A new sponsor can rate post-reset.
        let new_sponsor = Address::generate(&env);
        client.rate_planter(&new_sponsor, &planter, &5u32);
        let final_rep = client.get_reputation(&planter).unwrap();
        assert_eq!(final_rep.total_ratings, 4);
        // 12 + 5 = 17; 17 * 20 / 4 = 85
        assert_eq!(final_rep.average_rating, 85);
    }

    #[test]
    fn test_slash_does_not_affect_other_planters_availability() {
        let (env, admin, client) = setup();
        let a = Address::generate(&env);
        let b = Address::generate(&env);
        client.register_planter(&a, &land_hash(&env, 1), &String::from_str(&env, "s1"));
        client.register_planter(&b, &land_hash(&env, 2), &String::from_str(&env, "s2"));

        client.slash_planter(&admin, &a);

        assert!(!client.is_available(&a));
        assert!(client.is_available(&b));
    }

    #[test]
    fn test_slashed_planter_can_still_be_rated() {
        // A planter who has been slashed remains ratable — their reputation
        // record is preserved across the slash. This is "gating, not
        // forgetting".
        let (env, admin, client) = setup();
        let planter = Address::generate(&env);
        let sponsor = Address::generate(&env);

        client.register_planter(
            &planter,
            &land_hash(&env, 1),
            &String::from_str(&env, "s1"),
        );
        client.slash_planter(&admin, &planter);

        client.rate_planter(&sponsor, &planter, &4u32);
        let rep = client.get_reputation(&planter).unwrap();
        assert_eq!(rep.total_ratings, 1);
        assert_eq!(rep.average_rating, 80);
    }
}
