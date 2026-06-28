'use client';

import type { ReactNode } from 'react';
import { Button } from '@/components/atoms/Button';
import { Text } from '@/components/atoms/Text';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/molecules/Card';
import type { RetirementSelection } from '@/lib/types/retire';

interface RetirementConfirmationModalProps {
  isOpen: boolean;
  selection: RetirementSelection;
  onConfirm: () => void;
  onCancel: () => void;
  isLoading?: boolean;
}

export function RetirementConfirmationModal({
  isOpen,
  selection,
  onConfirm,
  onCancel,
  isLoading = false,
}: RetirementConfirmationModalProps): ReactNode {
  if (!isOpen) {
    return null;
  }

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-4"
      role="dialog"
      aria-modal="true"
      aria-labelledby="retirement-confirm-title"
    >
      <Card className="w-full max-w-lg border-destructive/40 bg-background">
        <CardHeader>
          <CardTitle className="text-lg" id="retirement-confirm-title">
            Confirm Retirement
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div
            className="rounded-lg border border-destructive/40 bg-destructive/10 p-3"
            role="alert"
          >
            <Text variant="small" as="p" className="font-semibold text-destructive">
              This action is permanent and cannot be undone.
            </Text>
            <Text variant="small" as="p" className="text-destructive/90">
              Retired credits are permanently removed from circulation and can never be traded or
              transferred again.
            </Text>
          </div>

          <div className="rounded-lg border border-border bg-muted/30 p-3">
            <div className="flex items-center justify-between">
              <Text variant="small" as="span" className="text-muted-foreground">
                Project
              </Text>
              <Text variant="small" as="span" className="font-semibold">
                {selection.projectName}
              </Text>
            </div>
            <div className="mt-2 flex items-center justify-between">
              <Text variant="small" as="span" className="text-muted-foreground">
                Quantity
              </Text>
              <Text variant="small" as="span" className="font-semibold">
                {selection.quantity.toLocaleString()} tCO₂e
              </Text>
            </div>
            <div className="mt-2 flex items-center justify-between">
              <Text variant="small" as="span" className="text-muted-foreground">
                Asset
              </Text>
              <Text variant="small" as="span" className="font-mono">
                {selection.assetCode}
              </Text>
            </div>
          </div>

          <div className="flex flex-col gap-3 sm:flex-row">
            <Button onClick={onCancel} disabled={isLoading} variant="outline" className="flex-1">
              Cancel
            </Button>
            <Button
              onClick={onConfirm}
              disabled={isLoading}
              className="flex-1 bg-destructive text-white hover:bg-destructive/90"
              aria-label="Confirm permanent retirement"
            >
              {isLoading ? 'Retiring...' : 'Retire Permanently'}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
