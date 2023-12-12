import Image from 'next/image'
import Link from 'next/link'
import { useRouter } from 'next/navigation'
import { ChangeEvent, useEffect, useRef, useState } from 'react'
import { fetchJsonFromPublic } from '../_utils/fetch-json'

type SearchResults = { cardName: string; image: string; slug: string }
export function Search({ className, overlayClass }: { className: string; overlayClass: string }) {
  const router = useRouter()
  const [searchQuery, setSearchQuery] = useState('')
  const [searchResults, setSearchResults] = useState<SearchResults[]>([])
  const [cursor, setCursor] = useState<number | null>(null)
  const [isSearchResultsHidden, setSearchResultsHidden] = useState<boolean>(false)
  const wrapperRef = useRef(null)
  useOutsideSearchHandler(wrapperRef, () => {
    setSearchResultsHidden(true)
    setCursor(null)
  })

  useEffect(() => {
    setCursor(null)
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
    <div ref={wrapperRef} className="w-full max-w-[524px]">
      <div className="flex">
        <input
          className={`${className} focus:outline-none`}
          value={cursor == null ? searchQuery : searchResults[cursor]?.cardName ?? searchQuery}
          onChange={(event: ChangeEvent<HTMLInputElement>) => setSearchQuery(event.target.value)}
          onClick={() => setSearchResultsHidden(false)}
          onKeyDown={(e) => {
            if (e.key === 'ArrowDown') {
              if (cursor == null) {
                setCursor(0)
              } else if (cursor >= searchResults.length - 1) {
                setCursor(null)
              } else {
                setCursor(cursor! + 1)
              }
            } else if (e.key === 'ArrowUp') {
              e.preventDefault()
              if (cursor === 0) {
                setCursor(null)
              } else if (cursor == null || cursor <= -1) {
                setCursor(searchResults.length - 1)
              } else {
                setCursor(cursor! - 1)
              }
            } else if (e.key === 'Enter' && cursor != null) {
              router.push(searchResults[cursor]?.slug)
            }
          }}
          placeholder="Search for Magic cards..."
        />
        <input
          value={
            cursor == null && !isSearchResultsHidden
              ? searchQuery +
                (searchResults[0]?.cardName.substring(searchQuery.length, searchResults[0].cardName.length) ?? '')
              : ''
          }
          onChange={() => {}}
          className={`${overlayClass} pointer-events-none bg-transparent border-transparent border-solid absolute focus:outline-none`}
          tabIndex={-1}
        />
      </div>
      {/* <div className="overflow-auto max-h-[200px] absolute bg-bg-color box-content rounded sm:translate-y-[26%] md:translate-y-[65px] lg:translate-y-[40px] w-full max-w-[524px]"> */}
      <div className="max-h-[200px] w-full max-w-[524px] relative">
        <div
          className="overflow-auto absolute bg-bg-color max-h-[200px] rounded w-full max-w-[524px]"
          hidden={isSearchResultsHidden}
        >
          {searchResults?.map((result, i) => {
            return (
              <Link
                href={`${result.slug}`}
                className={`${cursor === i ? 'bg-[#0A211C]' : ''} flex items-center`}
                key={i}
              >
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
      </div>
    </div>
  )
}

function useOutsideSearchHandler(ref: any, callback: () => void) {
  useEffect(() => {
    function handleClickOutside(event: any) {
      if (ref.current && !ref.current.contains(event.target)) {
        callback()
        console.log('You clicked outside of me!')
      }
    }

    document.addEventListener('mousedown', handleClickOutside)
    return () => {
      document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [ref, callback])
}

async function searchCard(cardName: string) {
  return await fetchJsonFromPublic<SearchResults[]>(`search/${cardName}`)
}
