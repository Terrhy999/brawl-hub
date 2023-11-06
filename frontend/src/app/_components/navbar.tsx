'use client'

import { ChangeEvent, MouseEventHandler, useEffect, useState } from 'react'
import Image from 'next/image'
import Link from 'next/link'
import { fetchJsonFromPublic } from '../_utils/fetch-json'
export function NavBar() {
  const [isHamburgerMenuHidden, setHamburgerMenuHidden] = useState(true)
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

type SearchResults = { cardName: string; image: string; slug: string }
export function SearchBar() {
  const [searchQuery, setSearchQuery] = useState('')
  const [searchResults, setSearchResults] = useState<SearchResults[]>()
  useEffect(() => {
    onSearch(searchQuery)
  }, [searchQuery])

  async function onSearch(searchQuery: string) {
    if (!searchQuery) {
      setSearchResults([])
      return
    }

    const searchResults = await searchCard(searchQuery)
    setSearchResults(searchResults)
  }
  return (
    <>
      <MagnifyingGlass className="ml-2" />
      <div className="h-full w-full ml-2">
        <input
          className="bg-header-color focus:outline-none w-full"
          value={searchQuery}
          onChange={(event: ChangeEvent<HTMLInputElement>) => setSearchQuery(event.target.value)}
          placeholder="Search for Magic cards..."
        />
        {/* (TODO) Figure out how to get this div to be the COMPUTED width of parent */}
        {/* <div className="overflow-auto max-h-[300px] absolute bg-bg-color box-content rounded translate-y-[16px] w-[53%] [border:1px_solid_rgba(0,0,0,0.4)]"> */}
        <div className="overflow-auto max-h-[300px] absolute bg-bg-color box-content rounded translate-y-[16px] w-[53%]">
          {searchResults?.map((result, i) => {
            return (
              <Link href={`${result.slug}`} key={i} className="flex items-center">
                <Image className="mr-2 w-auto h-auto" src={result.image} width={50} height={50} alt={result.cardName} />
                <div>{result.cardName}</div>
              </Link>
            )
          })}
        </div>
      </div>
    </>
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
      className={`w-[32px] ${className}`}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
      />
    </svg>
  )
}

async function searchCard(cardName: string) {
  return await fetchJsonFromPublic<SearchResults[]>(`search/${cardName}`)
}
