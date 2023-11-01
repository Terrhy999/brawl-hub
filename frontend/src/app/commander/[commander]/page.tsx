import { Card } from '@/app/_components/commander-card-gallery'
import { ClickableChip } from '@/app/commanders/layout'
import Image from 'next/image'
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
      <div
        style={{ ['--image-url' as any]: `url(${commanderCard.image_art_crop})` }}
        className={`absolute blur-md bg-cover w-full h-[442px] bg-[image:var(--image-url)]`}
      ></div>
      <div className="text-center z-40 relative">
        <h2 className="text-4xl">{commanderCard.name}</h2>
        <Card className="mx-auto my-2" card={commanderCard} size="large" />
        <div>
          <span className="text-accent-color">{commanderCard.count}</span> decks (%) of Decks with this Color Identity
        </div>
        <div>
          {' '}
          <span className="text-accent-color">{commanderCard.count}</span> decks (%) of ALL Decks
        </div>
        <div>Rank #</div>
      </div>

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
            <div className="grid grid-cols-[repeat(auto-fill,minmax(245px,1fr))] gap-y-5">
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
            </div>
          </div>
        ))}
      </div>
    </>
  )
}

function getPercentage(num1: number, num2: number): number {
  return Math.trunc((num1 / num2) * 100)
}

function Card({
  card,
  className = '',
  size = 'normal',
}: {
  card: Card
  className?: string
  size?: 'normal' | 'large'
}) {
  const width = size === 'normal' ? 244 : 336
  const height = size === 'normal' ? 340 : 468
  return (
    <Image
      className={`rounded-[5%] ${className}`}
      src={size === 'normal' ? card.image_normal : card.image_large}
      alt={card.name}
      width={width}
      height={height}
    />
  )
}
