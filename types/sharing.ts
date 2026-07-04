/**
 * Social sharing types and interfaces
 */

export type SharePlatform = 'twitter' | 'facebook' | 'linkedin' | 'whatsapp' | 'copy';

export interface ShareConfig {
  platform: SharePlatform;
  url: string;
  getShareUrl: (baseUrl: string, encodedText: string) => string;
  label: string;
  icon: string;
  color: string;
  ariaLabel: string;
}

export interface ShareOptions {
  url: string;
  title: string;
  description?: string;
  donationAmount?: number;
  impact?: string;
}

export interface ShareEvent {
  platform: SharePlatform;
  url: string;
  success: boolean;
  timestamp: number;
}

export interface ToastMessage {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
  duration?: number;
}

export interface ToastContextType {
  toasts: ToastMessage[];
  addToast: (message: string, type: 'success' | 'error' | 'info', duration?: number) => void;
  removeToast: (id: string) => void;
}
