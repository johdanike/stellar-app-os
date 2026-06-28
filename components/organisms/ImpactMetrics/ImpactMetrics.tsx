'use client';

import * as React from 'react';
import { Text } from '@/components/atoms/Text';
import { Card } from '@/components/molecules/Card';
import { TreePine, Wind, Sun, Droplets, Users, ShieldCheck } from 'lucide-react';
import { cn } from '@/lib/utils';

export interface ImpactMetric {
  label: string;
  value: string;
  unit?: string;
  icon: 'tree' | 'co2' | 'energy' | 'water' | 'community' | 'verified';
}

interface ImpactMetricsProps {
  metrics: ImpactMetric[];
  className?: string;
}

const iconMap = {
  tree: TreePine,
  co2: Wind,
  energy: Sun,
  water: Droplets,
  community: Users,
  verified: ShieldCheck,
};

export function ImpactMetrics({ metrics, className }: ImpactMetricsProps) {
  return (
    <div className={cn('grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4', className)}>
      {metrics.map((metric, index) => {
        const Icon = iconMap[metric.icon] || TreePine;
        return (
          <Card
            key={index}
            className="p-4 flex flex-col items-center text-center space-y-2 border-stellar-green/20 bg-stellar-green/5 hover:bg-stellar-green/10 transition-colors"
          >
            <div className="p-2 rounded-full bg-stellar-green/10 text-stellar-green">
              <Icon size={24} />
            </div>
            <div>
              <Text variant="h4" className="text-stellar-green font-bold">
                {metric.value}
                {metric.unit && (
                  <span className="text-xs ml-0.5 font-normal uppercase">{metric.unit}</span>
                )}
              </Text>
              <Text variant="small" className="text-muted-foreground leading-tight">
                {metric.label}
              </Text>
            </div>
          </Card>
        );
      })}
    </div>
  );
}
