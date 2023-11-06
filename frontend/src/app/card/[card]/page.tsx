import { CardPage } from '@/app/_components/card-page'
import { Card } from '@/app/_components/card-grid'
import Link from 'next/link'
import React from 'react'
import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'
import NotFound from '../not-found'

export const dynamicParams = false
export async function generateStaticParams() {
  // (TODO) find out why this is undefined sometimes
  return (await fetchJsonFromBrawlhub<string[]>(`card_slugs`)).map((name) => ({ card: name || 'not-found' }))
}

// async function getCommanderTopCards(oracle_id: string): Promise<TopCards> {
//   return await fetch(`http://127.0.0.1:3030/top_cards_for_commander/${oracle_id}`).then((res) => res.json())
// }

export default async function Page({ params = null }: { params: { card: string | 'not-found' } | null }) {
  const cardSlug = params?.card
  if (cardSlug == null || cardSlug == 'not-found') {
    return <NotFound />
  }
  const card = await fetchJsonFromBrawlhub<Card>(`card/${cardSlug}`)
  // const topCards = await getCommanderTopCards(commanderCard.oracle_id)
  return (
    <CardPage card={card}>
      <div className="max-w-[85%] mx-auto">
        <h3 id={'topCommanders'} className="text-3xl my-4 scroll-mt-16">
          <Link href={`#TopCommanders`} className="group">
            <span className="text-bg-color group-hover:text-accent-color">#</span> Top Commanders ()
          </Link>
        </h3>
      </div>
    </CardPage>
  )
}
