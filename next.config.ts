import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  output: 'standalone',
  outputFileTracingRoot: __dirname,
  experimental: {
    cpus: 1,
    staticGenerationMaxConcurrency: 1,
    staticGenerationMinPagesPerWorker: 25,
  },
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
  eslint: {
    // Warning: This allows production builds to successfully complete even if
    // your project has ESLint errors. Perfect for WIP branches.
    ignoreDuringBuilds: true,
  },
};

export default nextConfig;