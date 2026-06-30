#![no_std]

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

/// Donation escrow errors — used by donation-escrow only.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DonationEscrowError {
    UnsupportedToken = 71,
    EscrowNotFound = 72,
    AlreadyProcessed = 73,
    DonationCancelled = 74,
    IntervalNotElapsed = 75,
    RecurringDonationNotFound = 76,
    ProjectNotRegistered = 77,
    NotDonor = 78,
    DonationAlreadyCancelled = 79,
    AmountPerIntervalMustBePos = 80,
    IntervalSecondsMustBePos = 81,
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
