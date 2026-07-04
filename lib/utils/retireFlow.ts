import type { Step, StepStatus } from '@/components/molecules/ProgressStepper/ProgressStepper';

export const RETIRE_FLOW_STEPS = {
  SELECTION: {
    id: 'selection',
    label: 'Select',
    path: '/credits/retire',
  },
  WALLET: {
    id: 'wallet',
    label: 'Wallet',
    path: '/credits/retire/wallet',
  },
  CONFIRM: {
    id: 'confirm',
    label: 'Confirm',
    path: '/credits/retire/confirm',
  },
  CERTIFICATE: {
    id: 'certificate',
    label: 'Certificate',
    path: '/credits/retire/certificate',
  },
} as const;

export function getCurrentStepFromPath(pathname: string): string {
  if (pathname.includes('/certificate')) {
    return RETIRE_FLOW_STEPS.CERTIFICATE.id;
  }
  if (pathname.includes('/confirm')) {
    return RETIRE_FLOW_STEPS.CONFIRM.id;
  }
  if (pathname.includes('/wallet')) {
    return RETIRE_FLOW_STEPS.WALLET.id;
  }
  return RETIRE_FLOW_STEPS.SELECTION.id;
}

export function getStepStatus(
  stepId: string,
  currentStepId: string,
  completedSteps: string[]
): StepStatus {
  if (completedSteps.includes(stepId)) {
    return 'completed';
  }
  if (stepId === currentStepId) {
    return 'current';
  }
  return 'upcoming';
}

export function getCompletedSteps(
  currentStepId: string,
  hasSelection: boolean,
  hasWallet: boolean
): string[] {
  const completed: string[] = [];

  if (currentStepId !== RETIRE_FLOW_STEPS.SELECTION.id && hasSelection) {
    completed.push(RETIRE_FLOW_STEPS.SELECTION.id);
  }

  if (
    currentStepId !== RETIRE_FLOW_STEPS.WALLET.id &&
    (currentStepId === RETIRE_FLOW_STEPS.CONFIRM.id ||
      currentStepId === RETIRE_FLOW_STEPS.CERTIFICATE.id) &&
    hasWallet
  ) {
    completed.push(RETIRE_FLOW_STEPS.WALLET.id);
  }

  if (currentStepId === RETIRE_FLOW_STEPS.CERTIFICATE.id && hasSelection) {
    completed.push(RETIRE_FLOW_STEPS.CONFIRM.id);
  }

  return completed;
}

export function buildRetireFlowSteps(
  currentStepId: string,
  completedSteps: string[],
  selectionParam?: string | null
): Step[] {
  return [
    {
      ...RETIRE_FLOW_STEPS.SELECTION,
      status: getStepStatus(RETIRE_FLOW_STEPS.SELECTION.id, currentStepId, completedSteps),
      path: selectionParam
        ? `${RETIRE_FLOW_STEPS.SELECTION.path}?selection=${selectionParam}`
        : RETIRE_FLOW_STEPS.SELECTION.path,
    },
    {
      ...RETIRE_FLOW_STEPS.WALLET,
      status: getStepStatus(RETIRE_FLOW_STEPS.WALLET.id, currentStepId, completedSteps),
      path: selectionParam
        ? `${RETIRE_FLOW_STEPS.WALLET.path}?selection=${selectionParam}`
        : RETIRE_FLOW_STEPS.WALLET.path,
    },
    {
      ...RETIRE_FLOW_STEPS.CONFIRM,
      status: getStepStatus(RETIRE_FLOW_STEPS.CONFIRM.id, currentStepId, completedSteps),
      path: selectionParam
        ? `${RETIRE_FLOW_STEPS.CONFIRM.path}?selection=${selectionParam}`
        : RETIRE_FLOW_STEPS.CONFIRM.path,
    },
    {
      ...RETIRE_FLOW_STEPS.CERTIFICATE,
      status: getStepStatus(RETIRE_FLOW_STEPS.CERTIFICATE.id, currentStepId, completedSteps),
      path: RETIRE_FLOW_STEPS.CERTIFICATE.path,
    },
  ];
}
