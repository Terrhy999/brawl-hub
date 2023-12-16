'use client'

import { MouseEventHandler, useState } from 'react'
import Link from 'next/link'
import { Search } from './search'
export function NavBar() {
  const [isHamburgerMenuHidden, setHamburgerMenuHidden] = useState(true)
  return (
    <header className="md:h-[--header-height] bg-header-color sticky top-0 z-50">
      <div className="max-w-[85%] mx-auto flex py-4 whitespace-nowrap items-center">
        <Link href={'/'} className="hidden md:block">
          BrawlHub
        </Link>
        <Link href={'/'} className="md:hidden">
          BH
        </Link>
        <MagnifyingGlass className="ml-2 hidden sm:block" />
        <div className="h-full w-full ml-2">
          <Search
            className="bg-header-color w-full h-full"
            overlayClass="[color:hsla(0,0%,100%,.5)] bg-bg-color h-[24px]"
          />
        </div>
        <div className="[border-right:1px_solid_rgba(255,255,255,0.4)] mr-[6px] h-[20px] hidden md:block" />
        <nav className="[&>*]:px-[7px] hidden md:block">
          <Link href={'/commanders/'} className="!pl-[14px]">
            Commanders
          </Link>
          <Link href={'/cards/'}>Cards</Link>
          {/* <Link href={''}>Random</Link> */}
        </nav>
        <BarsSvg className="md:hidden" onClick={() => setHamburgerMenuHidden(!isHamburgerMenuHidden)}></BarsSvg>
      </div>
      <nav
        className={`${
          isHamburgerMenuHidden ? 'hidden' : 'block'
        } max-w-[85%] mx-auto [&>*]:w-[49%] [&>*]:mb-[8px] flex justify-between flex-wrap [border-top:1px_solid_rgba(255,255,255,0.4)] pt-[12px] pb-[8px] md:hidden`}
      >
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
        {/* <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Sets
        </Link> */}
        {/* <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Random
        </Link> */}
      </nav>
    </header>
  )
}

function MagnifyingGlass({ className }: { className: string }) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
      strokeWidth="1.5"
      stroke="currentColor"
      className={`w-[20px] ${className}`}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
      />
    </svg>
  )
}

function BarsSvg({
  className = '',
  onClick = undefined,
}: {
  className?: string
  onClick?: MouseEventHandler<SVGSVGElement> | undefined
}) {
  return (
    <svg
      onClick={onClick}
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
