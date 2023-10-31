import { Card } from '@/app/_components/commander-card-gallery'
import { ClickableChip } from '@/app/commanders/layout'
import Image from 'next/image'
import Link from 'next/link'
import React from 'react'

export const dynamicParams = false
const commander: Card = {
  color_identity: ['R'],
  cmc: 3,
  name: 'Chandra, Dressed To Kill',
  colors: ['R'],
  image_art_crop: 'https://cards.scryfall.io/art_crop/front/7/1/7129a358-4628-4426-ae3b-e3d9288a6355.jpg?1643597140',
  image_border_crop: '',
  image_large: 'https://cards.scryfall.io/large/front/7/1/7129a358-4628-4426-ae3b-e3d9288a6355.jpg?1643597140',
  image_normal: 'https://cards.scryfall.io/large/front/7/1/7129a358-4628-4426-ae3b-e3d9288a6355.jpg?1643597140',
  image_small: 'https://cards.scryfall.io/large/front/7/1/7129a358-4628-4426-ae3b-e3d9288a6355.jpg?1643597140',
  is_commander: true,
  is_legal: true,
  lang: 'eng',
  layout: 'normal',
  mana_cost: '1RR',
  oracle_id: '4686f776-2055-470c-bd99-7c4bb32902c0',
  oracle_text: 'o',
  rarity: 'mythic',
  scryfall_uri: '',
  type_line: 'Legendary Planeswalker - Chandra',
  count: 10,
}

type Sections =
  | 'newCards'
  | 'topCards'
  | 'creatures'
  | 'instants'
  | 'sorceries'
  | 'utilityArtifacts'
  | 'enchantments'
  | 'planeswalkers'
  | 'utilityLands'
  | 'manaArtifacts'
  | 'lands'

type MostUsedCards = Map<Sections, Card[]>

const data = new Map<string, string>()

export async function generateStaticParams() {
  function santisizeName(name: string): string {
    return name.replaceAll(',', '').replaceAll(' ', '-').toLocaleLowerCase()
  }
  const commandersName = (await getAllCommanders()).map((commander) => ({
    commander: santisizeName(commander.name),
  }))
  // commandersName.push({ commander: 'azusa-lost-but-seeking' })
  return commandersName
}

async function getAllCommanders(): Promise<Card[]> {
  // const yo = await fetch(`http://127.0.0.1:3030/top_cards_for_commander/7514e401-7aa1-405d-9f7a-312b4e630cc2`).then(
  //   (res) => res.json()
  // )
  // console.log(yo)
  return await fetch(`http://127.0.0.1:3030/all_commanders`).then((res) => res.json())
}

export default async function Page({ params }: { params: { commander: string } }) {
  const sections = [
    'Top Cards',
    'Creatures',
    'Instants',
    'Sorceries',
    'Utillity Artifacts',
    'Enchantments',
    'Planeswalkers',
    'Utility Lands',
    'Mana Artifacts',
    'Lands',
  ] as const
  const cards: string[] = new Array(20)
  const max = Math.floor(Math.random() * 20)
  cards.fill(commander.image_large, 0, 13)
  const artCrop = 'bg-[url(' + commander.image_art_crop + ')]'
  // const artCrop =
  //   'bg-[url(https://cards.scryfall.io/art_crop/front/7/1/7129a358-4628-4426-ae3b-e3d9288a6355.jpg?1643597140)]'
  return (
    <>
      {/* <div>
        <div className={`absolute blur-md bg-cover ${artCrop} w-full h-[442px]`}></div>
        <div className="relative z-40 max-w-[85%] pt-20 m-auto flex">
          <Card className="mr-5" card={commander} size="large" />
          <div className="mt-5 text-black">
            <h2 className="text-4xl">{commander.name}</h2>
            <div>Rank #</div>
            <div>{commander.count} decks (%) of ALL Decks</div>
            <div>{commander.count} decks (%) of Decks with this Color Identity</div>
          </div>
        </div>
</div> */}

      {/* maybe use semantic html for this */}
      <div className="max-w-[85%] mx-auto">
        <div className="flex sticky top-0 max-w-[1258px]">
          {sections.map((section, i) => (
            <ClickableChip key={i} className="mr-auto" text={section} href={`#${section.replace(' ', '_')}`} />
          ))}
        </div>
        <Section>
          <div className="grid grid-cols-[repeat(auto-fill,minmax(245px,1fr))] gap-y-5">
            {cards.map((_, i) => (
              // <div key={i} className="max-w-[245px] text-center">
              <div key={i} className="mx-auto text-center">
                <Card card={commander} />
                <div>{commander.count}% of decks</div>
                <div>+15% synergy</div>
              </div>
            ))}
          </div>
        </Section>
      </div>
    </>
  )
}

function Section({ children }: { children?: React.ReactNode }) {
  const sections = [
    'Top Cards',
    'Creatures',
    'Instants',
    'Sorceries',
    'Utillity Artifacts',
    'Enchantments',
    'Planeswalkers',
    'Utility Lands',
    'Mana Artifacts',
    'Lands',
  ] as const
  return (
    <>
      {sections.map((section, i) => (
        <>
          <h3 key={i} id={section.replace(' ', '_')} className="text-3xl my-4 scroll-mt-11">
            <Link href={`#${section.replace(' ', '_')}`}># {section}</Link>
          </h3>
          {children}
        </>
      ))}
    </>
  )
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
      src={card.image_large}
      alt={card.name}
      width={width}
      height={height}
    />
  )
}
