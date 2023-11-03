import { CardPage } from '@/app/_components/card-page'
import { Card } from '@/app/_components/card-grid'
import Link from 'next/link'
import React from 'react'
import { fetchJson } from '@/app/_utils/fetch-json'

export const dynamicParams = false
export async function generateStaticParams() {
  return (await fetchJson<string[]>(`http://127.0.0.1:3030/card_slugs`)).map((name) => ({ card: name }))
}

// async function getCommanderTopCards(oracle_id: string): Promise<TopCards> {
//   return await fetch(`http://127.0.0.1:3030/top_cards_for_commander/${oracle_id}`).then((res) => res.json())
// }

export default async function Page({ params }: { params: { card: string } }) {
  const cardSlug = params.card
  const card = await fetchJson<Card>(`http://127.0.0.1:3030/card/${cardSlug}`)
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
