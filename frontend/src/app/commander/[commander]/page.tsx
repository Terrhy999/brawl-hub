import CardGrid, { Card } from '@/app/_components/card-grid'
import { CommanderPage, CardSlug } from '@/app/_components/commander-page'
import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'
import { ClickableChip } from '@/app/_components/clickable-chip'
import Link from 'next/link'
import React from 'react'

export const dynamicParams = false
type TopCard = Card & {
  quantity: number
  total_commander_decks: number
  ci_quantity: number
  total_commander_decks_of_ci: number
  synergy: number
  usage_in_commander: number
  usage_in_color: number
}
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
  return (await fetchJsonFromBrawlhub<string[]>(`commander_slugs`)).map((name) => ({ commander: name }))
}

export default async function Page({ params }: { params: { commander: string } }) {
  const commanderSlug = params.commander
  const commanderCard = await fetchJsonFromBrawlhub<CardSlug>(`commander/${commanderSlug}`)
  const topCards = await fetchJsonFromBrawlhub<TopCards>(`commander_top_cards/${commanderCard.oracle_id}`)
  const sections = [
    // ['Top Cards', ],
    // (TODO) replace the _ with a -
    ['Creatures', 'creatures'],
    ['Instants', 'instants'],
    ['Sorceries', 'sorceries'],
    ['Utillity Artifacts', 'utility_artifacts'],
    ['Enchantments', 'enchantments'],
    ['Planeswalkers', 'planeswalkers'],
    ['Utility Lands', 'utility_lands'],
    ['Mana Artifacts', 'mana_artifacts'],
    ['Lands', 'lands'],
  ] as const
  return (
    <>
      <CommanderPage card={commanderCard}>
        <nav className="bg-bg-color sticky top-[--header-height] flex overflow-auto py-[10px] lg:max-w-[85%] lg:mx-auto">
          {sections.map((section, i) => (
            <ClickableChip key={i} className="mr-1" text={section[0]} href={`#${section[1]}`} />
          ))}
        </nav>
        <div className="max-w-[85%] mx-auto">
          {sections.map(([title, prop], i) => (
            <div key={i}>
              <h3 id={prop} className="text-3xl my-4 scroll-mt-16">
                <Link href={`#${prop}`} className="group">
                  <span className="text-bg-color group-hover:text-accent-color">#</span> {title} (
                  {topCards?.[prop]?.length ?? 0})
                </Link>
              </h3>
              <CardGrid cards={topCards?.[prop] ?? []}>{CardText}</CardGrid>
            </div>
          ))}
        </div>
      </CommanderPage>
    </>
  )
}

function CardText(card: TopCard) {
  return (
    <div className="text-center mt-1">
      <div>
        {card.quantity} of {card.total_commander_decks} decks ({Math.floor(card.usage_in_commander)}%)
      </div>
      <div>{Math.floor(card.synergy)}% synergy</div>
    </div>
  )
}
