import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import Link from 'next/link'
import { StrictMode } from 'react'

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

function NavBar() {
  return (
    <header className="h-[56px] bg-header-color">
      <div className="max-w-[85%] mx-auto flex items-center p-4 justify-between">
        <span>BrawlRec</span>
        <nav className="[&>*]:mr-5">
          <Link href={'/commanders/'}>Top Commanders</Link>
          <Link href={'/cards/'}>Top Cards</Link>
        </nav>
      </div>
    </header>
  )
}
