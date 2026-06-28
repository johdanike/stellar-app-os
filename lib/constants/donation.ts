export const MINIMUM_DONATION = 5;
export const TREES_PER_DOLLAR = 1;
export const HECTARES_PER_DOLLAR = 0.018;
export const CO2_PER_DOLLAR = 0.048;

// 30% of each donation is allocated to the replanting buffer fund.
// This pool covers tree failures and ensures survival rate targets are met.
export const REPLANTING_BUFFER_PERCENT = 0.3;

export interface DonationAllocation {
  total: number;
  planting: number; // 70% — direct tree planting
  buffer: number; // 30% — replanting buffer fund
}

export function calculateDonationAllocation(amount: number): DonationAllocation {
  const buffer = parseFloat((amount * REPLANTING_BUFFER_PERCENT).toFixed(7));
  const planting = parseFloat((amount - buffer).toFixed(7));
  return { total: amount, planting, buffer };
}

export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  }).format(amount);
}

export function formatNumber(num: number): string {
  return new Intl.NumberFormat('en-US').format(num);
}

export function generateIdempotencyKey(): string {
  return crypto.randomUUID();
}
