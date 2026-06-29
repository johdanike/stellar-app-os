#![no_std]

//! Shared error codes for all Harvesta / FarmCredit contracts.
//!
//! The Soroban SDK limits `#[contracterror]` enums to 50 cases. Errors are
//! split into three enums to stay within that limit:
//!
//! * `HarvestaError` — lifecycle, validation, registry, ZK, and escrow errors
//! * `GovernanceError` — multi-signature governance (admin-controls only)
//! * `MarketplaceError` — carbon marketplace / auction (carbon-marketplace only)

use soroban_sdk::contracterror;

/// General-purpose contract errors (45 variants — under the 50-case SDK limit).
///
/// NOTE: variants 65 and 66 are intentionally reused across domains
/// (farmer-registry, species-registry, location-proof).  Each contract only
/// panics with its own subset, so the codes are unambiguous in context.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum HarvestaError {
    // ── Common lifecycle (1–8) ─────────────────────────────────────────────────
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    ContractPaused = 4,
    AlreadyPaused = 5,
    NotPaused = 6,
    NoPendingAdmin = 7,
    ContractMustBeTreeTokenAdm = 8,

    // ── Amount / value validation (9–15) ──────────────────────────────────────
    AmountMustBePositive = 9,
    TreeCountMustBePositive = 10,
    FarmCountMustBePositive = 11,
    InvalidPayoutAmount = 13,
    BurnAmountMustBePositive = 14,
    SlotAmountMustBePositive = 15,

    // ── Farmer registry (35–37, 67-68) ───────────────────────────────────────────
    FarmerAlreadyRegistered = 35,
    FarmerNotRegistered = 36,
    InvalidRegion = 37,
    /// Caller is not a registered validator — gated read/write denied.
    NotValidator = 67,
    /// The SHA-256 hash supplied by the caller does not match the one stored
    /// on-chain for this farmer's identity document.
    HashMismatch = 68,

    // ── Species registry (62–64, 69-70) ──────────────────────────────────────────────
    Co2MustBePositive = 62,
    MaturityYearsMustBePositive = 63,
    SpeciesNotFound = 64,
    InvasiveSpecies = 69,
    HighWaterUse = 70,

    // ── Carbon marketplace (100–113) ───────────────────────────────────────────
    ListingAmountMustBePositive = 100,
    PriceMustBePositive = 101,
    ListingNotFound = 102,
    ListingNotActive = 103,
    InsufficientLiquidity = 104,
    BuyAmountMustBePositive = 105,
    SelfTrade = 106,
    InvalidPriceRange = 107,
    InvalidDecayRate = 108,
    InvalidDuration = 109,
    AuctionNotFound = 110,
    AuctionNotActive = 111,
    AuctionExpired = 112,
    BidBelowReservePrice = 113,

    // ── Arithmetic overflows (86–87) ──────────────────────────────────────────
    TreeTokenMintOverflow = 86,
    TokenUnitOverflow = 87,
}

/// Multi-signature governance errors — used by admin-controls only.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GovernanceError {
    MultisigNotInitialized = 82,
    NotASigner = 83,
    ProposalNotFound = 84,
    ProposalAlreadyExecuted = 85,
    AlreadyApproved = 86,
    ThresholdTooHigh = 87,
    ThresholdMustBePositive = 88,
    SignerAlreadyExists = 89,
    SignerNotFound = 90,
    MinimumOneSignerRequired = 91,
}

/// Carbon marketplace / auction errors — used by carbon-marketplace only.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MarketplaceError {
    ListingAmountMustBePositive = 100,
    PriceMustBePositive = 101,
    ListingNotFound = 102,
    ListingNotActive = 103,
    InsufficientLiquidity = 104,
    BuyAmountMustBePositive = 105,
    SelfTrade = 106,
    InvalidPriceRange = 107,
    InvalidDecayRate = 108,
    InvalidDuration = 109,
    AuctionNotFound = 110,
    AuctionNotActive = 111,
    AuctionExpired = 112,
    BidBelowReservePrice = 113,
}
