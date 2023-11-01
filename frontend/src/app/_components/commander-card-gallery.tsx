import Image from 'next/image'
import Link from 'next/link'

export type Card = {
  oracle_id: string // Assuming Uuid is represented as a string
  name: string
  lang: string
  scryfall_uri: string
  layout: string
  mana_cost: string | null
  cmc: number
  type_line: string
  oracle_text: string | null
  colors: (string | null)[]
  color_identity: string[]
  is_legal: boolean
  is_commander: boolean
  rarity: string
  image_small: string
  image_normal: string
  image_large: string
  image_art_crop: string
  image_border_crop: string
  count?: number | null
  slug: string
}

export default async function CommanderCardGallery({ commanders }: { commanders: Card[] }) {
  return (
    <div className="grid gap-y-5 grid-cols-[repeat(auto-fill,minmax(245px,1fr))]">
      {commanders.map((card, i: number) => (
        <Link key={i} href={`commander/${card.slug}`}>
          <CardAndRank i={i} card={card} />
        </Link>
      ))}
    </div>
  )
}

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
