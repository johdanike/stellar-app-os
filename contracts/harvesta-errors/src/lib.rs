#![no_std]

//! Shared error codes for all Harvesta / FarmCredit contracts.
//!
//! Import the crate, then call `panic_with_error!(env, HarvestaError::Variant)`
//! instead of raw string panics.  Error codes are stable u32 values embedded in
//! the Stellar XDR so off-chain tooling can parse them without string matching.

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

    // ── Amount / value validation (9–15) ──────────────────────────────────────
    AmountMustBePositive = 9,
    TreeCountMustBePositive = 10,
    VerifiedCountMustBePositive = 11,
    VerifiedCountExceedsDon = 12,
    InvalidPayoutAmount = 13,
    BurnAmountMustBePositive = 14,
    SlotAmountMustBePositive = 15,

    // ── Escrow state (16–25) ──────────────────────────────────────────────────
    EscrowAlreadyExists = 16,
    EscrowNotFound = 17,
    PlantingAlreadyVerified = 18,
    PlantingNotVerified = 19,
    RefundAfterPlanting = 20,
    SurvivalThresholdOutOfRange = 21,
    SurvivalRateOutOfRange = 22,
    SurvivalRateBelowMinimum = 23,
    SurvivalPeriodNotElapsed = 24,
    NothingToRelease = 25,

    // ── Oracle / tree co-fund (26–34) ─────────────────────────────────────────
    UnauthorizedOracle = 26,
    NoOracleReport = 27,
    BatchEmpty = 28,
    BatchTooLarge = 29,
    TreeAlreadyRegistered = 30,
    TreeNotRegistered = 31,
    TreeNotOpenForContributions = 32,
    TreeNotOpenForRelease = 33,
    NoFundsToRelease = 34,

    // ── Farmer registry (35–37) ───────────────────────────────────────────────
    FarmerAlreadyRegistered = 35,
    FarmerNotRegistered = 36,
    InvalidRegion = 37,

    // ── Dispute / arbiter (38–46) ─────────────────────────────────────────────
    DisputeAlreadyOpen = 38,
    NoOpenDispute = 39,
    EscrowAlreadyFinalised = 40,
    NotArbiter = 41,
    NotBuyerOrSeller = 42,
    MilestoneReleaseBlocked = 43,
    MilestoneAlreadyProcessed = 44,
    CompletionPercentOutOfRange = 45,
    TotalReleasedExceedsMile = 46,

    // ── Naira payout (47–54) ──────────────────────────────────────────────────
    PendingPayoutAlreadyExists = 47,
    PayoutIntervalTooShort = 48,
    MaxDailyPayoutExceeded = 49,
    PayoutNotPending = 50,
    CanOnlyCancelPending = 51,
    PayoutNotFound = 52,
    ExpectedNgnMustBePositive = 53,
    UnsupportedToken = 54,

    // ── Aggregate impact verifier (55–59) ─────────────────────────────────────
    FarmCountMustBePositive = 55,
    PeriodEndBeforeStart = 56,
    ProofDigestAlreadyReg = 57,
    ProofNotFound = 58,
    ProofAlreadyRevoked = 59,

    // ── Nullifier registry (60) ───────────────────────────────────────────────
    CommitmentAlreadyRegistered = 60,

    // ── KYC attestation (61) ──────────────────────────────────────────────────
    NotVerifier = 61,

    // ── Species registry (62–64) ──────────────────────────────────────────────
    Co2MustBePositive = 62,
    MaturityYearsMustBePositive = 63,
    SpeciesNotFound = 64,

    // ── Location proofs (65–66) ───────────────────────────────────────────────
    OutsideNigeriaRegion = 65,
    ProofCommitmentAlreadyReg = 66,

    // ── Farmer registry validator gates (67–68) ──────────────────────────────
    NotValidator = 67,
    HashMismatch = 68,

    // ── Species policy (69–70) ───────────────────────────────────────────────
    InvasiveSpecies = 69,
    HighWaterUse = 70,

    // ── Donation escrow (71–79) ───────────────────────────────────────────────
    AlreadyProcessed = 71,
    NotDonor = 72,
    DonationAlreadyCancelled = 73,
    DonationCancelled = 74,
    IntervalNotElapsed = 75,
    ProjectNotRegistered = 76,
    AmountPerIntervalMustBePos = 77,
    IntervalSecondsMustBePos = 78,
    RecurringDonationNotFound = 79,

    // ── Donation escrow auto-refund (#634) (82–83) ────────────────────────────
    MilestoneDeadlineNotPassed = 82,
    LocationAlreadyVerified = 83,

    // ── Tree registry (88–90) ─────────────────────────────────────────────────
    NotFound = 88,
    InvalidStatus = 89,
    NotAuthorized = 90,

    // ── Verifier staking (91–95) ──────────────────────────────────────────────
    MinStakeMustBePositive = 91,
    VerifierAlreadyStaked = 92,
    VerifierNotStaked = 93,
    SlashExceedsStake = 94,
    InsufficientStake = 95,

    // ── Carbon marketplace (100–113) ──────────────────────────────────────────
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

    // ── ZK location verifier (120–123) ─────────────────────────────────────────
    CommitmentAlreadySubmitted = 120,
    CommitmentNotFound = 121,
    CommitmentNotPending = 122,
    InvalidProof = 123,

    // ── Farm Plots (150–151) ──────────────────────────────────────────────────
    InvalidCoordinatesCount = 150,
    PlotAlreadyExists = 151,

    // ── Arithmetic overflows (86–87) ──────────────────────────────────────────
    TreeTokenMintOverflow = 86,
    TokenUnitOverflow = 87,
}
