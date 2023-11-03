'use client'

import { ChangeEvent, useEffect, useState } from 'react'
import Image from 'next/image'
import { fetchJson } from '../_utils/fetch-json'
import Link from 'next/link'

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
        <div className="overflow-auto max-h-[300px] absolute bg-bg-color box-content rounded translate-y-[16px] w-[53%] [border:1px_solid_rgba(0,0,0,0.4)]">
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
  return await fetchJson<SearchResults[]>(`http://127.0.0.1:3030/search/${cardName}`)
}
