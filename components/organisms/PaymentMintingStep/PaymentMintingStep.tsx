'use client';

import { useState, useCallback } from 'react';
import { Button } from '@/components/atoms/Button';
import { Text } from '@/components/atoms/Text';
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from '@/components/molecules/Card';
import { useWalletContext } from '@/contexts/WalletContext';
import { signTransactionWithFreighter, signTransactionWithAlbedo } from '@/lib/stellar/signing';
import { mockCarbonProjects } from '@/lib/api/mock/carbonProjects';
import type { PaymentMintingProps, TransactionStatus } from '@/lib/types/payment';
import { useToast } from '@/contexts/ToastContext';

export function PaymentMintingStep({
  selection,
  wallet,
  onComplete,
  onError,
}: PaymentMintingProps) {
  const { refreshBalance } = useWalletContext();
  const { toast } = useToast();
  const [status, setStatus] = useState<TransactionStatus>('idle');
  const [transactionHash, setTransactionHash] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);

  const selectedProject = mockCarbonProjects.find((p) => p.id === selection.projectId);

  const generateIdempotencyKey = useCallback(() => {
    return `${Date.now()}-${Math.random().toString(36).substring(2, 15)}`;
  }, []);

  const handleSignTransaction = useCallback(async () => {
    if (!wallet || !selection.projectId || selection.quantity <= 0) {
      setError('Invalid selection or wallet not connected');
      return;
    }

    setIsProcessing(true);
    setError(null);
    setStatus('preparing');

    try {
      const idempotencyKey = generateIdempotencyKey();

      setStatus('preparing');
      const buildResponse = await fetch('/api/transaction/build', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          selection,
          walletPublicKey: wallet.publicKey,
          network: wallet.network,
          idempotencyKey,
        }),
      });

      if (!buildResponse.ok) {
        const errorData = (await buildResponse.json()) as { error?: string };
        throw new Error(errorData.error || 'Failed to build transaction');
      }

      const { transactionXdr, networkPassphrase } = (await buildResponse.json()) as {
        transactionXdr: string;
        networkPassphrase: string;
      };

      setStatus('signing');
      let signedXdr: string;

      if (wallet.type === 'freighter') {
        signedXdr = await signTransactionWithFreighter(transactionXdr, networkPassphrase);
      } else if (wallet.type === 'albedo') {
        signedXdr = await signTransactionWithAlbedo(transactionXdr, wallet.network);
      } else {
        throw new Error('Unsupported wallet type');
      }

      setStatus('submitting');
      const submitResponse = await fetch('/api/transaction/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          signedTransactionXdr: signedXdr,
          network: wallet.network,
        }),
      });

      if (!submitResponse.ok) {
        const errorData = (await submitResponse.json()) as { error?: string };
        throw new Error(errorData.error || 'Failed to submit transaction');
      }

      const { transactionHash: hash } = (await submitResponse.json()) as {
        transactionHash: string;
      };

      setTransactionHash(hash);
      setStatus('confirming');

      toast.info(
        'Transaction Submitted',
        'Your payment transaction has been submitted to the Stellar network and is being confirmed.'
      );

      await new Promise((resolve) => setTimeout(resolve, 2000));

      setStatus('success');
      await refreshBalance();

      if (onComplete) {
        onComplete(hash);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Transaction failed';
      setError(errorMessage);
      setStatus('error');
      if (onError) {
        onError(errorMessage);
      }
    } finally {
      setIsProcessing(false);
    }
  }, [selection, wallet, generateIdempotencyKey, refreshBalance, onComplete, onError, toast]);

  const canProceed =
    wallet?.isConnected &&
    selection.projectId &&
    selection.quantity > 0 &&
    selection.calculatedPrice > 0 &&
    parseFloat(wallet.balance.usdc) >= selection.calculatedPrice;

  const hasInsufficientBalance =
    wallet?.isConnected &&
    selection.calculatedPrice > 0 &&
    parseFloat(wallet.balance.usdc) < selection.calculatedPrice;

  // Show loading state while redirecting to confirmation page
  if (status === 'success' && transactionHash) {
    return (
      <div className="space-y-6">
        <div className="text-center">
          <div className="flex justify-center mb-4">
            <div className="h-12 w-12 border-4 border-stellar-green border-t-transparent rounded-full animate-spin" />
          </div>
          <Text variant="h3" as="h2" className="mb-2">
            Transaction Successful!
          </Text>
          <Text variant="muted" as="p">
            Redirecting to confirmation page...
          </Text>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <Text variant="h3" as="h2" className="mb-2">
          Review & Confirm Payment
        </Text>
        <Text variant="muted" as="p">
          Review your transaction details before signing.
        </Text>
      </div>

      {error && status === 'error' && (
        <div className="rounded-lg border border-destructive bg-destructive/10 p-4" role="alert">
          <Text variant="small" as="p" className="text-destructive font-semibold mb-1">
            Transaction Failed
          </Text>
          <Text variant="small" as="p" className="text-destructive">
            {error}
          </Text>
          <Button
            variant="outline"
            size="sm"
            className="mt-3"
            onClick={() => {
              setError(null);
              setStatus('idle');
            }}
          >
            Try Again
          </Button>
        </div>
      )}

export interface TransactionPreview {
  projectName: string;
  quantity: number;
  pricePerTon: number;
  totalAmount: number;
  paymentAsset: string;
  recipientAddress: string;
}

export interface PaymentMintingProps {
  selection: CreditSelectionState;
  wallet: WalletConnection | null;
  onComplete?: (transactionHash: string) => void;
  onError?: (error: string) => void;
}

export interface BuildTransactionRequest {
  selection: CreditSelectionState;
  walletPublicKey: string;
  network: 'testnet' | 'mainnet';
  idempotencyKey: string;
}

export interface BuildTransactionResponse {
  transactionXdr: string;
  networkPassphrase: string;
}
