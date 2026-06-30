import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  typescript: { ignoreBuildErrors: true },
  staticPageGenerationTimeout: 300,
  experimental: {
    // Keep page batching optimization (not a throttle).
    staticGenerationMinPagesPerWorker: 25,
  },
  output: 'standalone',
  outputFileTracingRoot: __dirname,
  images: {
    remotePatterns: [
      {
        // Allow any HTTPS image source — the CMS will provide the actual domain.
        // Restrict this to specific domains once the CMS URL is known.
        protocol: 'https',
        hostname: '**',
      },
    ],
  },
};

export default nextConfig;
