import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'
import Link from 'next/link'
import { StrictMode } from 'react'
import { SearchBar } from './_components/search-bar'

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
    <header className="md:h-[--header-height] bg-header-color sticky top-0 z-50">
      <div className="max-w-[85%] mx-auto flex py-4 whitespace-nowrap items-center">
        <span className="hidden md:block">BrawlHub</span>
        <span className="md:hidden">BH</span>
        <SearchBar />
        <div className="[border-right:1px_solid_rgba(255,255,255,0.4)] mr-[6px] h-[20px] hidden md:block" />
        <nav className="[&>*]:px-[7px] hidden md:block">
          <Link href={'/commanders/'} className="!pl-[14px]">
            Commanders
          </Link>
          <Link href={'/cards/'}>Cards</Link>
          <Link href={''}>Random</Link>
        </nav>
        <BarsSvg className="md:hidden"></BarsSvg>
      </div>
      <nav className="max-w-[85%] mx-auto [&>*]:w-[49%] [&>*]:mb-[8px] flex justify-between flex-wrap [border-top:1px_solid_rgba(255,255,255,0.4)] pt-[12px] pb-[8px] md:hidden">
        <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Commanders
        </Link>
        <Link
          href={'/cards/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Cards
        </Link>
        <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Sets
        </Link>
        <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Random
        </Link>
      </nav>
    </header>
  )
}

function BarsSvg({ className }: { className: string }) {
  return (
    <svg
      // onClick={}
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      strokeWidth="1.5"
      stroke="currentColor"
      className={`w-6 h-6 ${className}`}
    >
      <path strokeLinecap="round" strokeLinejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
    </svg>
  )
}
