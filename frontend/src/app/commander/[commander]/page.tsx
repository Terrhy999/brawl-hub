import { Card } from '@/app/_components/commander-card-gallery'
import Image from 'next/image'
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
  const cards: string[] = new Array(20)
  const max = Math.floor(Math.random() * 20)
  cards.fill(commander.image_large, 0, 8)
  const artCrop = "bg-[url('" + commander.image_art_crop + "')]"
  return (
    <>
      <div>
        <div className={`absolute blur-sm bg-cover ${artCrop} w-full h-[442px]`}></div>
        {/* <div className={`absolute blur-sm bg-cover ${artCrop} w-full h-[400px]`}></div> */}
        <div className="relative z-40 max-w-[75%] m-auto pt-[120px] flex">
          {/* <Image className="rounded-[5%]" src={commander.image_large} alt={commander.name} width={244} height={340} /> */}
          <Image
            className="rounded-[5%] mr-5"
            src={commander.image_large}
            alt={commander.name}
            width={336}
            height={468}
          />
          <div className="mt-5 text-black">
            <h2 className="text-4xl">{commander.name}</h2>
            <div>Rank #</div>
            <div>{commander.count} decks (%) of ALL Decks</div>
            <div>{commander.count} decks (%) of Decks with this Color Identity</div>
          </div>
        </div>
      </div>

      {/* maybe use semantic html for this */}
      <div className="max-w-[75%] m-auto">
        <Section>
          <div className="grid grid-cols-[repeat(auto-fill,minmax(245px,1fr))] mb-5">
            {cards.map((card, i) => (
              <div key={i}>
                <Image className="rounded-[5%]" src={card} alt={card} width={244} height={340} />
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
          <h3 key={i} className="text-3xl mb-4">
            {section}
          </h3>
          {children}
        </>
      ))}
    </>
  )
}
