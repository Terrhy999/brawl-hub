/** @type {import('next').NextConfig} */
const nextConfig = {
    images: {
        // formats: ['image/webp, image/jpg'],
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'cards.scryfall.io',
                port: '',
            }
        ]
    }
}

module.exports = nextConfig
