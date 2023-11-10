'use client'
import Link from 'next/link'
import { ChangeEvent, useEffect, useState } from 'react'
import Image from 'next/image'
import { fetchJsonFromPublic } from './_utils/fetch-json'

export default function Home() {
  return (
    <div className="px-[5%] flex justify-center items-center h-screen flex-col [background:linear-gradient(to_top,#213B20,#1B341F,#162F1E,#132B1E,#10281D,#0A211C)]">
      <h1 className="text-[24px] md:text-[32px] text-center mb-[20px]">
        BrawlHub is the #1 spot to get rec&apos;s for your Historic Brawl decks!
      </h1>
      {/* <input
        className="[border:1px_solid_rgba(255,255,255,0.25)] bg-[#0A211C] py-[12px] pr-[14px] pl-[14px] w-full max-w-[524px] focus:outline-none"
        placeholder="Search for Magic cards..."
      /> */}
      <SearchBar />
      <nav className={`flex justify-between pt-[12px] mb-[200px]`}>
        <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)] mr-2"
        >
          Top Commanders
        </Link>
        <Link
          href={'/cards/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Top Cards
        </Link>
      </nav>
    </div>
  )
}

type SearchResults = { cardName: string; image: string; slug: string }
function SearchBar() {
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
      <input
        className="[border:1px_solid_rgba(255,255,255,0.25)] bg-[#0A211C] py-[12px] px-[14px] w-full max-w-[524px] focus:outline-none"
        value={searchQuery}
        onChange={(event: ChangeEvent<HTMLInputElement>) => setSearchQuery(event.target.value)}
        placeholder="Search for Magic cards..."
      />
      {/* <div className="overflow-auto max-h-[200px] absolute bg-bg-color box-content rounded sm:translate-y-[26%] md:translate-y-[65px] lg:translate-y-[40px] w-full max-w-[524px]"> */}
      <div className="overflow-auto max-h-[200px] rounded w-full max-w-[524px] bg-bg-color">
        {searchResults?.map((result, i) => {
          return (
            <Link href={`${result.slug}`} key={i} className="flex items-center">
              <Image
                className="mr-2 h-[40px] w-[50px]"
                src={result.image}
                width={50}
                height={50}
                alt={result.cardName}
              />
              <div>{result.cardName}</div>
            </Link>
          )
        })}
      </div>
    </>
  )
}

async function searchCard(cardName: string) {
  return await fetchJsonFromPublic<SearchResults[]>(`search/${cardName}`)
}
