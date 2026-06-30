'use client';

import { Text } from '@/components/atoms/Text';
import type { PaymentMintingProps, TransactionStatus } from '@/lib/types/payment';

// Re-export surface types from the canonical source so adjacent files
// (notably the barrel `index.tsx` in this folder, plus any tests) can
// still import them from `@/components/organisms/PaymentMintingStep`.
export type { PaymentMintingProps, TransactionStatus };

/**
 * Review-and-confirm step in the carbon credit purchase wizard.
 *
 * Renders the static "Review & Confirm Payment" header only. The actual
 * signing and submission flow lives in the parent wizard so it can own
 * the CTAs, balance gating, and navigation back to the confirmation
 * page. This component is intentionally presentational — it accepts
 * no props and exposes no hooks.
 */
export function PaymentMintingStep() {
  return (
    <div className="space-y-6">
      <div>
        <Text variant="h3" as="h2" className="mb-2">
          Review &amp; Confirm Payment
        </Text>
        <Text variant="muted" as="p">
          Review your transaction details before signing.
        </Text>
      </div>
    </div>
  );
}
