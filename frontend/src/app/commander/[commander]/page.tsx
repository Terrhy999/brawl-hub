import CardGrid, { Card } from '@/app/_components/card-grid'
import { CardPage } from '@/app/_components/card-page'
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
  const commanderNames: string[] = await fetch(`http://127.0.0.1:3030/commander_slugs`).then((res) => res.json())
  return commanderNames.map((name) => {
    commander: name
  })
}

async function getCommanderBySlug(commanderSlug: string): Promise<Card> {
  return await fetch(`http://127.0.0.1:3030/commander/${commanderSlug}`).then((res) => res.json())
}

async function getCommanderTopCards(oracle_id: string): Promise<TopCards> {
  return await fetch(`http://127.0.0.1:3030/top_cards_for_commander/${oracle_id}`).then((res) => res.json())
}

export default async function Page({ params }: { params: { commander: string } }) {
  const commanderSlug = params.commander
  const commanderCard = await getCommanderBySlug(commanderSlug)
  const topCards = await getCommanderTopCards(commanderCard.oracle_id)
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
      <CardPage card={commanderCard}>
        <nav className="bg-bg-color sticky top-0 flex overflow-auto py-[10px] lg:max-w-[85%] lg:mx-auto">
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
              <CardGrid cards={topCards?.[prop] ?? []}>{CardText(commanderCard)}</CardGrid>
            </div>
          ))}
        </div>
      </CardPage>
    </>
  )
}

function CardText(commanderCard: Card): (card: TopCard) => React.ReactNode {
  return function Text(card: TopCard) {
    return (
      <div>
        <span className="text-accent-color">{getPercentage(card.num_decks_with_card, commanderCard?.count ?? 1)}%</span>
        of {commanderCard.count}
      </div>
    )
  }
}

function getPercentage(num1: number, num2: number): number {
  return Math.trunc((num1 / num2) * 100)
}
