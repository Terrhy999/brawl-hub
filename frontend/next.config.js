/** @type {import('next').NextConfig} */
const nextConfig = {
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
}

module.exports = nextConfig
