import { CardSlug } from '@/app/_components/commander-page'
import Link from 'next/link'
import React from 'react'
import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'
import { Card, CardGridWithText } from '@/app/_components/card-grid'
import { CardPage } from '@/app/_components/card-page'

// export const dynamicParams = false
// export async function generateStaticParams() {
//   return (await fetchJsonFromBrawlhub<string[]>(`card_slugs`)).map((name) => ({ card: name }))
// }

async function getTopCommanders(slug: string): Promise<Card[]> {
  return await fetchJsonFromBrawlhub<Card[]>(`top_commanders_for_card/${slug}`)
}

export default async function Page({ params }: { params: { card: string } }) {
  const cardSlug = params.card
  const card = await fetchJsonFromBrawlhub<CardSlug>(`card/${cardSlug}`)
  const topCommanders = await getTopCommanders(card.slug)
  return (
    <CardPage card={card}>
      <div className="max-w-[85%] mx-auto">
        <h3 id={'topCommanders'} className="text-3xl my-4 scroll-mt-16">
          <Link href={`#TopCommanders`} className="group">
            <span className="text-bg-color group-hover:text-accent-color">#</span> Top Commanders (
            {topCommanders.length})
          </Link>
        </h3>
        <CardGridWithText linkTo="commander" cards={topCommanders} />
      </div>
    </CardPage>
  )
}
