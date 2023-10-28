import { Card, ClickableChip } from '@/app/page'
import Image from 'next/image'
import Link from 'next/link'

export const dynamicParams = false
const colorCombinations = [
  { colorIdentity: 'w', title: 'Mono-White' },
  { colorIdentity: 'u', title: 'Mono-Blue' },
  { colorIdentity: 'b', title: 'Mono-Black' },
  { colorIdentity: 'r', title: 'Mono-Red' },
  { colorIdentity: 'g', title: 'Mono-Green' },
  { colorIdentity: 'wu', title: 'Azorious' },
  { colorIdentity: 'ub', title: 'Dimir' },
  { colorIdentity: 'br', title: 'Rakdos' },
  { colorIdentity: 'rg', title: 'Gruul' },
  { colorIdentity: 'gw', title: 'Selesnya' },
  { colorIdentity: 'wb', title: 'Orzhov' },
  { colorIdentity: 'ur', title: 'Izzet' },
  { colorIdentity: 'bg', title: 'Golgari' },
  { colorIdentity: 'rw', title: 'Boros' },
  { colorIdentity: 'gu', title: 'Simic' },
  { colorIdentity: 'wub', title: 'Esper' },
  { colorIdentity: 'ubr', title: 'Grixis' },
  { colorIdentity: 'brg', title: 'Jund' },
  { colorIdentity: 'rgw', title: 'Naya' },
  { colorIdentity: 'gwu', title: 'Bant' },
  { colorIdentity: 'wbg', title: 'Abzan' },
  { colorIdentity: 'urw', title: 'Jeskai' },
  { colorIdentity: 'bgu', title: 'Sultai' },
  { colorIdentity: 'rwb', title: 'Mardu' },
  { colorIdentity: 'gur', title: 'Temur' },
  { colorIdentity: 'wubr', title: 'Sans-Green' },
  { colorIdentity: 'ubrg', title: 'Sans-White' },
  { colorIdentity: 'brgw', title: 'Sans-Blue' },
  { colorIdentity: 'rgwu', title: 'Sans-Black' },
  { colorIdentity: 'gwub', title: 'Sans-Red' },
  { colorIdentity: 'wubrg', title: 'Five-Color' },
  { colorIdentity: 'colorless', title: 'Colorless' },
] as const

export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

async function getCommandersByColorIdentity(colorIdentity: string): Promise<Card[]> {
  const commandersOfColors: Card[] = await fetch(`http://127.0.0.1:3030/commanders/${colorIdentity}`).then((res) =>
    res.json()
  )
  return commandersOfColors
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const { colorIdentity } = params
  const colorCombinationName = colorCombinations.find((combo) => colorIdentity === combo.colorIdentity)?.title ?? ''
  let activeDateFilter = 'year'
  const top_commanders = await getCommandersByColorIdentity(params.colorIdentity)
  return (
    // <div className="bg-[#22262a] text-white">
    <main>
      <h1 className="text-[32px]">Top {colorCombinationName} Commanders</h1>
      <div className="flex justify-between mb-5">
        {/* Change this to a radio button */}
        <span className="flex [&>*]:mr-[12px]">
          <ClickableChip text={'Year'} isActive={activeDateFilter === 'year'} />
          <ClickableChip text={'Month'} isActive={activeDateFilter === 'month'} />
          <ClickableChip text={'Week'} isActive={activeDateFilter === 'week'} />
        </span>

        <div className="flex [&>*]:mr-[20px] [&>a]:opacity-30 [&>*]:duration-[0.3s]">
          <Link href={'/commanders/w'} className={`${params.colorIdentity === 'w' ? 'opacity-[unset]' : ''}`}>
            <Image src={'/white-mana-symbol.png'} alt={'White mana'} width={36} height={36} />
          </Link>
          <Image src={'/blue-mana-symbol.png'} alt={'Blue mana'} width={36} height={36} />
          <Image src={'/black-mana-symbol.png'} alt={'Black mana'} width={36} height={36} />
          <Image src={'/red-mana-symbol.png'} alt={'Red mana'} width={36} height={36} />
          <Image src={'/green-mana-symbol.png'} alt={'Green mana'} width={36} height={36} />
        </div>
      </div>

      <div className="grid gap-[20px] grid-cols-[repeat(auto-fit,minmax(270px,1fr))]">
        {top_commanders.map((card, i: number) => (
          <CardAndRank key={i} i={i} card={card} />
        ))}
      </div>
    </main>
  )
}

// replace 'i' with count
function CardAndRank({ i, card }: { i: number; card: Card }) {
  return (
    <div className="flex flex-col items-center">
      <Image className="rounded-[5%] max-h-[340px]" src={card.image_large} alt={card.name} width={244} height={340} />
      <div className="text-center">
        <div>Rank #{i + 1}</div>
        <div>{card.count} decks</div>
      </div>
    </div>
  )
}
