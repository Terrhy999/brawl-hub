import Link from 'next/link'
import { Card } from '../_components/commander-card-gallery'
import Image from 'next/image'

async function getTopCards(): Promise<Card[]> {
  const res = await fetch('http://127.0.0.1:3030/top_cards', {
    cache: 'no-cache',
  })
  return res.json()
}

export default async function Page() {
  const topCards = await getTopCards()
  return (
    <div className="grid gap-y-5 grid-cols-[repeat(auto-fill,minmax(245px,1fr))]">
      {topCards.map((card, i: number) => (
        <Link key={i} href={`card/${card.slug}`}>
          <div className="flex flex-col items-center">
            <Card card={card} />
            <div className="text-center">
              <div>Rank #{i + 1}</div>
              <div>{card.count} decks</div>
            </div>
          </div>
        </Link>
      ))}
    </div>
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
      src={size === 'normal' ? card.image_normal : card.image_large}
      alt={card.name}
      width={width}
      height={height}
    />
  )
}
