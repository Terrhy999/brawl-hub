/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  images: {
    unoptimized: true,
    // formats: ['image/webp, image/jpg'],
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'cards.scryfall.io',
        port: '',
      },
    ],
  },
  staticPageGenerationTimeout: 1000,
}

module.exports = nextConfig
