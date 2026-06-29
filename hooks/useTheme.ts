'use client';
<<<<<<< HEAD
import { useState, useEffect } from 'react';

export function useTheme() {
  const [theme, setTheme] = useState<'light' | 'dark'>(() => {
    if (typeof window === 'undefined') return 'dark';
    const stored = localStorage.getItem('theme');
    if (stored === 'light' || stored === 'dark') return stored;
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  });

  useEffect(() => {
    document.documentElement.classList.toggle('dark', theme === 'dark');
    localStorage.setItem('theme', theme);
  }, [theme]);

  useEffect(() => {
    const stored = localStorage.getItem('theme');
    if (stored) return;
    const mq = window.matchMedia('(prefers-color-scheme: dark)');
    const handler = (e: MediaQueryListEvent) => setTheme(e.matches ? 'dark' : 'light');
    mq.addEventListener('change', handler);
    return () => mq.removeEventListener('change', handler);
  }, []);

  const toggle = () => setTheme((t) => (t === 'dark' ? 'light' : 'dark'));

  return { theme, toggle, isDark: theme === 'dark' };
=======

import { useCallback, useEffect, useState, useSyncExternalStore } from 'react';
import {
  applyThemeToDocument,
  getStoredTheme,
  getSystemPrefersDark,
  persistTheme,
  resolveTheme,
  type ResolvedTheme,
} from '@/lib/theme';
import type { Theme } from '@/types/settings';

function subscribeToSystemTheme(onStoreChange: () => void): () => void {
  const mq = window.matchMedia('(prefers-color-scheme: dark)');
  mq.addEventListener('change', onStoreChange);
  return () => mq.removeEventListener('change', onStoreChange);
}

function getSystemThemeSnapshot(): ResolvedTheme {
  return getSystemPrefersDark() ? 'dark' : 'light';
}

function getServerThemeSnapshot(): ResolvedTheme {
  return 'dark';
}

export function useTheme() {
  const [theme, setThemeState] = useState<Theme>(() => getStoredTheme());
  const systemTheme = useSyncExternalStore(
    subscribeToSystemTheme,
    getSystemThemeSnapshot,
    getServerThemeSnapshot
  );

  const resolvedTheme: ResolvedTheme =
    theme === 'system' ? systemTheme : resolveTheme(theme, systemTheme === 'dark');

  useEffect(() => {
    applyThemeToDocument(theme);
    persistTheme(theme);
  }, [theme, systemTheme]);

  const setTheme = useCallback((next: Theme) => {
    setThemeState(next);
  }, []);

  const toggle = useCallback(() => {
    setThemeState(resolvedTheme === 'dark' ? 'light' : 'dark');
  }, [resolvedTheme]);

  return {
    theme,
    resolvedTheme,
    setTheme,
    toggle,
    isDark: resolvedTheme === 'dark',
  };
>>>>>>> 4fa2ff0e46c01b84d0a39c3524e33dea37e50005
}
