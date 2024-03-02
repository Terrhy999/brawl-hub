import Image from 'next/image'
import Link from 'next/link'
import { FlipCard, HyperLinkedFlipCard } from './flip-card'

export type Card = {
  oracle_id: string
  name_full: string
  name_front: string
  name_back: string | null
  slug: string
  scryfall_uri: string
  layout: string
  rarity: string
  lang: string
  mana_cost_combined: string | null
  mana_cost_front: string | null
  mana_cost_back: string | null
  cmc: number
  type_line_full: string
  type_line_front: string
  type_line_back: string | null
  oracle_text: string | null
  oracle_text_back: string | null
  colors: (string | null)[]
  colors_back: (string | null)[]
  color_identity: string[]
  is_legal: boolean
  is_legal_commander: boolean
  is_rebalanced: boolean
  image_small: string
  image_normal: string
  image_large: string
  image_art_crop: string
  image_border_crop: string
  image_small_back?: string | null
  image_normal_back?: string | null
  image_large_back?: string | null
  image_art_crop_back?: string | null
  image_border_crop_back?: string | null
  // count?: number | null
}

export type TopCard = Card & {
  total_decks_with_card?: number
  total_decks_could_play?: number
  rank?: number
}

export type CardCount = Card & {
  count?: number | null
}

type CardProps = {
  card: Card
  className?: string
  size?: 'normal' | 'large'
  cardFace?: 'front' | 'back'
}

type LinkTarget = 'commander' | 'card'

export default async function CardGrid({
  cards,
  linkTo = 'card',
  children,
}: {
  cards: Card[] | CardCount[] | TopCard[]
  linkTo?: LinkTarget
  children?: (...args: any[]) => React.ReactNode
}) {
  return (
    <div className="grid gap-y-5 grid-cols-[repeat(auto-fill,minmax(245px,1fr))]">
      {cards.map((card, i) => (
        <div key={i}>
          <div className="flex flex-col items-center">{renderHyperlinkedCard(card, linkTo)}</div>
          {children != undefined ? children(card, i) : undefined}
        </div>
      ))}
    </div>
  )
}

export async function CardGridWithText({
  cards,
  linkTo = 'card',
}: {
  cards: Card[] | CardCount[] | TopCard[]
  linkTo?: LinkTarget
}) {
  return (
    <CardGrid cards={cards} linkTo={linkTo}>
      {CardText}
    </CardGrid>
  )
}

export function CardImage({ card, className = '', size = 'normal', cardFace = 'front' }: CardProps) {
  const width = size === 'normal' ? 244 : 336
  const height = size === 'normal' ? 340 : 468
  let src = size === 'normal' ? card.image_normal : card.image_large
  if (cardFace === 'back') {
    src = size === 'normal' ? card?.image_normal_back ?? '' : card?.image_large_back ?? ''
  }
  return <Image className={`rounded-[5%] ${className}`} src={src} alt={card.name_full} width={width} height={height} />
}

function CardText(card: TopCard, i: number): React.ReactNode {
  return (
    <div className="text-center">
      <div>Rank #{i + 1}</div>
      <div>
        In {card.total_decks_with_card} of {card.total_decks_could_play} decks ({Math.round(card.rank!)}%)
      </div>
    </div>
  )
}

function renderHyperlinkedCard(card: Card, linkTo: LinkTarget) {
  if (card?.image_normal_back == null) {
    return (
      <Link href={`/${linkTo}/${card.slug}`}>
        <Card card={card} />
      </Link>
    )
  }
  return <HyperLinkedFlipCard card={card} linkTo={linkTo} />
}

export function Card({ card, className, size = 'normal' }: CardProps) {
  if (card?.image_normal_back == null) {
    return <CardImage card={card} className={className} size={size} />
  }
  return <FlipCard card={card} className={className} size={size} />
}
