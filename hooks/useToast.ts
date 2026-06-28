'use client';

import { useContext } from 'react';
import { ToastContext } from '@/components/providers/ToastProvider';
import { type ToastContextType } from '@/types/sharing';

/**
 * Hook to use toast notifications
 */
export function useToast(): ToastContextType {
  const context = useContext(ToastContext);

  if (!context) {
    throw new Error('useToast must be used within ToastProvider');
  }

  return context;
}
