import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'BrawlRec',
  description: 'Find your next Historic Brawl Commander!',
}

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-bg-color text-white`}>
        <NavBar />
        <main className="mx-auto">{children}</main>
      </body>
    </html>
  )
}

function NavBar() {
  return (
    <header className="h-[56px] bg-header-color flex items-center p-4">
      BrawlRec
      <nav></nav>
    </header>
  )
}
