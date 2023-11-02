import { Card } from '@/app/_components/card-grid'
import { ClickableChip } from '@/app/commanders/layout'
import Link from 'next/link'
import React from 'react'

export const dynamicParams = false

type TopCard = Card & { num_decks_with_card: number; num_decks_total: number }

type Sections =
  | 'newCards'
  | 'topCards'
  | 'creatures'
  | 'instants'
  | 'sorceries'
  | 'utility_artifacts'
  | 'enchantments'
  | 'planeswalkers'
  | 'utility_lands'
  | 'mana_artifacts'
  | 'lands'

type TopCards = Record<Sections, TopCard[]>

export async function generateStaticParams() {
  const commanderNames: string[] = await fetch(`http://127.0.0.1:3030/card_slugs`).then((res) => res.json())
  return commanderNames.map((name) => {
    card: name
  })
}

async function getCommanderBySlug(cardSlug: string): Promise<Card> {
  return await fetch(`http://127.0.0.1:3030/card/${cardSlug}`).then((res) => res.json())
}

// async function getCommanderTopCards(oracle_id: string): Promise<TopCards> {
//   return await fetch(`http://127.0.0.1:3030/top_cards_for_commander/${oracle_id}`).then((res) => res.json())
// }

export default async function Page({ params }: { params: { card: string } }) {
  const cardSlug = params.card
  const card = await getCommanderBySlug(cardSlug)
  // const topCards = await getCommanderTopCards(commanderCard.oracle_id)
  return (
    <>
      <div
        style={{ ['--image-url' as any]: `url(${card.image_art_crop})` }}
        className={`absolute blur-md bg-cover w-full h-[442px] bg-[image:var(--image-url)]`}
      ></div>
      <div className="text-center z-40 relative">
        <h2 className="text-4xl">{card.name}</h2>
        <Card className="mx-auto my-2" card={card} size="large" />
        <div>
          <span className="text-accent-color">{card.count}</span> decks (%) of Decks with this Color Identity
        </div>
        <div>
          <span className="text-accent-color">{card.count}</span> decks (%)
        </div>
        <div>Rank #</div>
      </div>

      <div className="max-w-[85%] mx-auto">
        <h3 id={'topCommanders'} className="text-3xl my-4 scroll-mt-16">
          <Link href={`#TopCommanders`} className="group">
            <span className="text-bg-color group-hover:text-accent-color">#</span> Top Commanders ()
          </Link>
        </h3>
        {/* <div className="grid grid-cols-[repeat(auto-fill,minmax(245px,1fr))] gap-y-5">
              {topCards?.[prop]?.map((card, i) => (
                <div key={i} className="mx-auto text-center">
                  <Card card={card} />
                  <div>
                    <span className="text-accent-color">
                      {getPercentage(card.num_decks_with_card, commanderCard?.count ?? 1)}%
                    </span>{' '}
                    of {commanderCard.count} decks
                  </div>
                </div>
              ))}
            </div> */}
      </div>
    </>
  )
}
