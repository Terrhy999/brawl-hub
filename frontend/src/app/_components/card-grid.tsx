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

export default async function CardGrid({
  cards,
  linkTo = 'card',
  children,
}: {
  cards: Card[]
  linkTo?: 'commander' | 'card'
  // figure out how to type this properly so the user knows it can take (card: Card, i: number)
  // but also keep it generic enough that it doesnt need to
  // children?: (card: Card, i: number) => React.ReactNode
  children?: (...args: any[]) => React.ReactNode
}) {
  return (
    <div className="grid gap-y-5 grid-cols-[repeat(auto-fill,minmax(245px,1fr))]">
      {cards.map((card, i) => (
        <Link key={i} href={`/${linkTo}/${card.slug}`}>
          <div className="flex flex-col items-center">
            <Card card={card} />
            {children != undefined ? children(card, i) : undefined}
          </div>
        </Link>
      ))}
    </div>
  )
}

export function Card({
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
