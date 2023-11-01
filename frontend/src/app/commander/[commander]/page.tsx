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
      {/* <div className="sticky top-0 bg-[#1E1E1E] h-100 py-4 max-w-[85%] mx-auto"> */}
      <nav className="bg-[#1E1E1E] sticky top-0 flex overflow-auto py-[10px] lg:max-w-[85%] lg:mx-auto">
        {sections.map((section, i) => (
          <ClickableChip key={i} className="mr-1" text={section[0]} href={`#${section[1]}`} />
        ))}
      </nav>
      <div className="max-w-[85%] mx-auto">
        {sections.map(([title, prop], i) => (
          <div key={i}>
            <h3 id={prop} className="text-3xl my-4 scroll-mt-16">
              <Link href={`#${prop}`}>
                # {title} ({topCards?.[prop]?.length ?? 0})
              </Link>
            </h3>
            <div className="grid grid-cols-[repeat(auto-fill,minmax(245px,1fr))] gap-y-5">
              {topCards?.[prop]?.map((card, i) => (
                <div key={i} className="mx-auto text-center">
                  <Card card={card} />
                  <div>
                    {getPercentage(card.num_decks_with_card, commanderCard?.count ?? 1)}% of {commanderCard.count} decks
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

// export default async function Page({ params }: { params: { commander: string } }) {
// const cards: string[] = new Array(20)
// const max = Math.floor(Math.random() * 20)
// cards.fill(commander.image_large, 0, 13)
// const artCrop = 'bg-[url(' + commander.image_art_crop + ')]'
// // const artCrop =
// //   'bg-[url(https://cards.scryfall.io/art_crop/front/7/1/7129a358-4628-4426-ae3b-e3d9288a6355.jpg?1643597140)]'
// return (
//   <>
//     {/* <div>
//       <div className={`absolute blur-md bg-cover ${artCrop} w-full h-[442px]`}></div>
//       <div className="relative z-40 max-w-[85%] pt-20 m-auto flex">
//         <Card className="mr-5" card={commander} size="large" />
//         <div className="mt-5 text-black">
//           <h2 className="text-4xl">{commander.name}</h2>
//           <div>Rank #</div>
//           <div>{commander.count} decks (%) of ALL Decks</div>
//           <div>{commander.count} decks (%) of Decks with this Color Identity</div>
//         </div>
//       </div>
//     </div> */}

//     {/* maybe use semantic html for this */}
//     <div className="sticky top-0 bg-[#1E1E1E] h-100 py-4 max-w-[85%] mx-auto">
//       {sections.map((section, i) => (
//         <ClickableChip key={i} className="mr-1 mb-2" text={section} href={`#${section.replace(' ', '_')}`} />
//       ))}
//     </div>
//     <div className="max-w-[85%] mx-auto">
//       <Section>
//         <div className="grid grid-cols-[repeat(auto-fill,minmax(245px,1fr))] gap-y-5">
//           {/* {topCardsArr.map((top, i) => (
//             <div key={i}>{topCards?.[top]?.map((card, i) => <Card key={i} card={card} />)}</div>
//           ))} */}
//           {/* {cards.map((_, i) => (
//             // <div key={i} className="max-w-[245px] text-center">
//             <div key={i} className="mx-auto text-center">
//               <Card card={commander} />
//               <div>{commander.count}% of decks</div>
//               <div>+15% synergy</div>
//             </div>
//           ))} */}
//         </div>
//       </Section>
//     </div>
//   </>
// )
// }
