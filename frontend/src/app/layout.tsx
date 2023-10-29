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
      {/* <body className={`${inter.className} bg-[#0f0f0f] text-white`}> */}
      {/* <body className={`${inter.className} bg-[#141d26] text-white`}> */}
      <body className={`${inter.className} bg-[#1E1E1E] text-white`}>
        <NavBar />
        {children}
      </body>
    </html>
  )
}

function NavBar() {
  return (
    // <header className="h-[56px] bg-[#a2ac94] flex items-center p-4">
    <header className="h-[56px] bg-[#000000] opacity-20 flex items-center p-4">
      BrawlRec
      <nav></nav>
    </header>
  )
}
