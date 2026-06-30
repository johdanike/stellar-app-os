#![no_std]

use soroban_sdk::contracterror;

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

    // ── Amount / value validation (9–13) ──────────────────────────────────────
    ValueMustBePositive = 9,
    VerifiedCountExceedsDonation = 10,
    InvalidPayoutAmount = 11,

    // ── Escrow state (12–19) ──────────────────────────────────────────────────
    EscrowAlreadyExists = 12,
    EscrowNotFound = 13,
    PlantingNotVerified = 14,
    RefundAfterPlanting = 15,
    SurvivalRateOutOfRange = 16,
    SurvivalRateBelowMinimum = 17,
    SurvivalPeriodNotElapsed = 18,
    NothingToRelease = 19,

    // ── Dispute / arbiter (20–28) ─────────────────────────────────────────────
    DisputeAlreadyOpen = 20,
    NoOpenDispute = 21,
    EscrowAlreadyFinalised = 22,
    NotArbiter = 23,
    NotBuyerOrSeller = 24,
    MilestoneReleaseBlocked = 25,
    MilestoneAlreadyProcessed = 26,
    CompletionPercentageOutOfRange = 27,
    TotalReleasedExceedsMilestone = 28,

    // ── Farmer registry (29–33) ───────────────────────────────────────────────
    FarmerAlreadyRegistered = 29,
    FarmerNotRegistered = 30,
    InvalidRegion = 31,
    NotValidator = 32,
    HashMismatch = 33,

    // ── Species registry (34–37) ──────────────────────────────────────────────
    SpeciesNotFound = 34,
    InvasiveSpecies = 35,
    HighWaterUse = 36,

    // ── Carbon marketplace (37–45) ────────────────────────────────────────────
    ListingNotFound = 37,
    ListingNotActive = 38,
    InsufficientLiquidity = 39,
    SelfTrade = 40,
    InvalidPriceRange = 41,
    AuctionNotFound = 42,
    AuctionNotActive = 43,
    AuctionExpired = 44,
    BidBelowReservePrice = 45,

    // ── Farmer registry (35–37, 67-68) ───────────────────────────────────────────
    FarmerAlreadyRegistered = 35,
    FarmerNotRegistered = 36,
    InvalidRegion = 37,
    /// Caller is not a registered validator — gated read/write denied.
    NotValidator = 67,
    /// The SHA-256 hash supplied by the caller does not match the one stored
    /// on-chain for this farmer's identity document.
    HashMismatch = 68,
    // ── Farm Plots (150–151) ──────────────────────────────────────────────
    InvalidCoordinatesCount = 150,
    PlotAlreadyExists = 151,

    // ── Species registry (62–64, 69-70) ──────────────────────────────────────────────
    Co2MustBePositive = 62,
    MaturityYearsMustBePositive = 63,
    SpeciesNotFound = 64,
    InvasiveSpecies = 67,
    HighWaterUse = 68,
    InvasiveSpecies = 69,
    HighWaterUse = 70,

    // ── Arithmetic overflows (48–49) ──────────────────────────────────────────
    TreeTokenMintOverflow = 48,
    TokenUnitOverflow = 49,

    // ── Multi-party consensus (50–52) ─────────────────────────────────────────
    NotAVerifier = 50,
    AlreadyVoted = 51,
    VerifierAlreadyRegistered = 52,
    // ── Tree registry (88–90) ─────────────────────────────────────────────────
    NotFound = 88,
    InvalidStatus = 89,
    NotAuthorized = 90,

    // ── Arithmetic overflows (86–87) ──────────────────────────────────────────
    TreeTokenMintOverflow = 86,
    TokenUnitOverflow = 87,
}
