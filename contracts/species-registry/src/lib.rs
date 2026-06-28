#![no_std]

//! Species Registry — Closes #554, #484
//!
//! On-chain catalogue of tree species with FAO/IPCC Tier-1 CO₂ sequestration
//! rates, native region, and growth metadata. Used by frontend dropdowns.
//! The off-chain seeder (`scripts/seed-species.mjs`) calls
//! `register_species` for each row in `data/fao_co2_rates.csv`.
//!
//! # Storage layout
//!   Instance:
//!     ADMIN          — Address   (admin allowed to register/update species)
//!   Persistent (keyed by species slug Symbol):
//!     species:<slug> — SpeciesRecord
//!
//! # Functions
//!   initialize(admin)
//!   register_species(slug, name, native_region, co2_scaled, growth_rate_scaled, maturity_years)   — admin only
//!   get_species(slug) -> SpeciesRecord
//!   get_co2_rate(slug) -> i128   (co2_kg_per_year × 100)
//!   list_species(slugs) -> Vec<SpeciesRecord>

use harvesta_errors::HarvestaError;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, symbol_short, Address, Env, String,
    Symbol, Vec,
};

// ── Types ─────────────────────────────────────────────────────────────────────

/// On-chain record for a single species — closes #484.
///
/// All rate fields use scaled integers to avoid floats on-chain:
/// * `co2_scaled`          = kg CO₂/year × 100   (e.g. 2200 = 22.00 kg/yr)
/// * `growth_rate_scaled`  = cm height/year × 10  (e.g. 125  = 12.5 cm/yr)
#[contracttype]
#[derive(Clone, Debug)]
pub struct SpeciesRecord {
    /// Short identifier used as the storage key, e.g. `Symbol::new(&env, "teak")`.
    pub slug: Symbol,
    /// Human-readable common name, e.g. "Teak".
    pub name: String,
    /// Native geographical region, e.g. "South and Southeast Asia".
    pub native_region: String,
    /// CO₂ kg/year × 100 (scaled integer, must be > 0).
    pub co2_scaled: i128,
    /// Annual height growth in cm × 10 (scaled integer, must be > 0).
    pub growth_rate_scaled: i128,
    /// Years to biomass maturity (must be > 0).
    pub maturity_years: u32,
    /// Ledger timestamp of last update.
    pub updated_at: u64,
}

// ── Storage keys ──────────────────────────────────────────────────────────────

fn admin_key() -> Symbol {
    symbol_short!("ADMIN")
}

fn species_key(slug: &Symbol) -> (Symbol, Symbol) {
    (symbol_short!("SPECIES"), slug.clone())
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct SpeciesRegistry;

#[contractimpl]
impl SpeciesRegistry {
    /// One-time initialisation.  Must be called before any other function.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&admin_key()) {
            panic_with_error!(&env, HarvestaError::AlreadyInitialized);
        }
        env.storage().instance().set(&admin_key(), &admin);
    }

    /// Register or update a species.  Caller must be the stored admin.
    ///
    /// * `slug`               — short identifier, e.g. `Symbol::new(&env, "teak")`
    /// * `name`               — human-readable common name, e.g. "Teak"
    /// * `native_region`      — geographical origin, e.g. "South and Southeast Asia"
    /// * `co2_scaled`         — kg CO₂ per year × 100  (positive integer)
    /// * `growth_rate_scaled` — cm height/year × 10     (positive integer)
    /// * `maturity_years`     — years to biomass maturity
    pub fn register_species(
        env: Env,
        slug: Symbol,
        name: String,
        native_region: String,
        co2_scaled: i128,
        growth_rate_scaled: i128,
        maturity_years: u32,
    ) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&admin_key())
            .unwrap_or_else(|| panic_with_error!(&env, HarvestaError::NotInitialized));
        admin.require_auth();

        if co2_scaled <= 0 {
            panic_with_error!(&env, HarvestaError::Co2MustBePositive);
        }
        if growth_rate_scaled <= 0 {
            panic_with_error!(&env, HarvestaError::GrowthRateMustBePositive);
        }
        if maturity_years == 0 {
            panic_with_error!(&env, HarvestaError::MaturityYearsMustBePositive);
        }

        let record = SpeciesRecord {
            slug: slug.clone(),
            name,
            native_region,
            co2_scaled,
            growth_rate_scaled,
            maturity_years,
            updated_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&species_key(&slug), &record);

        env.events().publish(
            (symbol_short!("species"), symbol_short!("register")),
            (slug, co2_scaled, maturity_years),
        );
    }

    /// Retrieve the full record for a species slug.  Panics if not found.
    pub fn get_species(env: Env, slug: Symbol) -> SpeciesRecord {
        env.storage()
            .persistent()
            .get(&species_key(&slug))
            .unwrap_or_else(|| panic_with_error!(&env, HarvestaError::SpeciesNotFound))
    }

    /// Convenience: return only the scaled CO₂ rate for a species.
    pub fn get_co2_rate(env: Env, slug: Symbol) -> i128 {
        let record: SpeciesRecord = env
            .storage()
            .persistent()
            .get(&species_key(&slug))
            .unwrap_or_else(|| panic_with_error!(&env, HarvestaError::SpeciesNotFound));
        record.co2_scaled
    }

    /// Batch-retrieve species records by slug list (for frontend dropdowns).
    /// Panics if any slug is not found.
    pub fn list_species(env: Env, slugs: Vec<Symbol>) -> Vec<SpeciesRecord> {
        let mut out: Vec<SpeciesRecord> = Vec::new(&env);
        for i in 0..slugs.len() {
            let slug = slugs.get(i).unwrap();
            let record: SpeciesRecord = env
                .storage()
                .persistent()
                .get(&species_key(&slug))
                .unwrap_or_else(|| panic_with_error!(&env, HarvestaError::SpeciesNotFound));
            out.push_back(record);
        }
        out
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, vec, Env, String as SorobanString, Symbol};

    fn reg(client: &SpeciesRegistryClient, slug: &Symbol, env: &Env) {
        client.register_species(
            slug,
            &SorobanString::from_str(env, "Teak"),
            &SorobanString::from_str(env, "South and Southeast Asia"),
            &2200_i128,
            &125_i128,
            &20_u32,
        );
    }

    #[test]
    fn test_register_and_get() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SpeciesRegistry);
        let client = SpeciesRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let slug = Symbol::new(&env, "teak");
        reg(&client, &slug, &env);

        let record = client.get_species(&slug);
        assert_eq!(record.co2_scaled, 2200);
        assert_eq!(record.growth_rate_scaled, 125);
        assert_eq!(record.maturity_years, 20);
        assert_eq!(client.get_co2_rate(&slug), 2200);
    }

    #[test]
    fn test_list_species_returns_batch() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SpeciesRegistry);
        let client = SpeciesRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let s1 = Symbol::new(&env, "teak");
        let s2 = Symbol::new(&env, "acacia");
        reg(&client, &s1, &env);
        client.register_species(
            &s2,
            &SorobanString::from_str(&env, "Acacia"),
            &SorobanString::from_str(&env, "Africa"),
            &1500_i128,
            &80_i128,
            &10_u32,
        );

        let slugs = vec![&env, s1.clone(), s2.clone()];
        let records = client.list_species(&slugs);
        assert_eq!(records.len(), 2);
        assert_eq!(records.get(0).unwrap().slug, s1);
        assert_eq!(records.get(1).unwrap().slug, s2);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #64)")]
    fn test_get_unknown_species_panics() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SpeciesRegistry);
        let client = SpeciesRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let slug = Symbol::new(&env, "unknown");
        client.get_species(&slug);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #62)")]
    fn test_reject_zero_co2() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SpeciesRegistry);
        let client = SpeciesRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.register_species(
            &Symbol::new(&env, "bad"),
            &SorobanString::from_str(&env, "Bad"),
            &SorobanString::from_str(&env, "Nowhere"),
            &0_i128,
            &100_i128,
            &5_u32,
        );
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #82)")]
    fn test_reject_zero_growth_rate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SpeciesRegistry);
        let client = SpeciesRegistryClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);
        client.register_species(
            &Symbol::new(&env, "bad"),
            &SorobanString::from_str(&env, "Bad"),
            &SorobanString::from_str(&env, "Nowhere"),
            &100_i128,
            &0_i128,
            &5_u32,
        );
    }
}
