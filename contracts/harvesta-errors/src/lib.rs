

#![no_std]

// You must import contracterror from the soroban_sdk
use soroban_sdk::contracterror;

#[contracterror] // This should now resolve correctly
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum HarvestaError {
    // ... (Keep existing 1–8)

    // ── Amount / value validation (9–15) ──────────────────────────────────────
    // ... (Keep existing 9–15)

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

    // ── Dispute / arbiter (38–46) ─────────────────────────────────────────────
    DisputeAlreadyOpen = 38,
    NoOpenDispute = 39,
    EscrowAlreadyFinalised = 40,
    NotArbiter = 41,
    NotBuyerOrSeller = 42,
    MilestoneReleaseBlocked = 43,
    MilestoneAlreadyProcessed = 44,
    CompletionPercentageOutOfRange = 45,
    TotalReleasedExceedsMilestone = 46,

    // ... (Keep existing 62–81)
    // ── Farmer registry (35–37) ───────────────────────────────────────────────
    // FarmerAlreadyRegistered = 35,
    // FarmerNotRegistered = 36,
    // InvalidRegion = 37,

    // ── Dispute / arbiter (38–46) ─────────────────────────────────────────────
    // DisputeAlreadyOpen = 38,
    // NoOpenDispute = 39,
    // EscrowAlreadyFinalised = 40,
    // NotArbiter = 41,
    // NotBuyerOrSeller = 42,
    // MilestoneReleaseBlocked = 43,
    // MilestoneAlreadyProcessed = 44,
    // CompletionPercentageOutOfRange = 45,
    // TotalReleasedExceedsMilestone = 46,

    // ── Species registry (62–64) ──────────────────────────────────────────────
    Co2MustBePositive = 62,
    MaturityYearsMustBePositive = 63,
    SpeciesNotFound = 64,

    // ── Arithmetic overflows (80–81) ──────────────────────────────────────────
    TreeTokenMintOverflow = 80,
    TokenUnitOverflow = 81,
}
