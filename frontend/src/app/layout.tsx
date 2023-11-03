import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import { StrictMode } from 'react'
import { NavBar } from './_components/navbar'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'BrawlRec',
  description: 'Find your next Historic Brawl Commander!',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <StrictMode>
      <html lang="en">
        <body className={`${inter.className} bg-bg-color text-white`}>
          <NavBar />
          <main className="mx-auto">{children}</main>
        </body>
      </html>
    </StrictMode>
  )
}
