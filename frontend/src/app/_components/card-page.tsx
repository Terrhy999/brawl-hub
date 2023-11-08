import { Card } from './card-grid'

export type CardSlug = Card & {
  total_decks: number
  all_decks: number
  rank: number
  total_commander_decks_of_ci: number
}

export function CardPage({ card, children }: { card: CardSlug; children: React.ReactNode }) {
  console.log(card)
  return (
    <>
      <div
        style={{ ['--image-url' as any]: `url(${card.image_art_crop})` }}
        className={`absolute blur-md bg-cover w-full h-[442px] bg-[image:var(--image-url)]`}
      ></div>
      <div className="text-center z-40 relative">
        <h2 className="text-4xl">{card.name}</h2>
        <Card className="mx-auto my-2" card={card} size="large" />
        {/* <div>
          {Math.floor((card.total_decks / card.total_commander_decks_of_ci) * 100)}% decks with this Color Identity
        </div> */}
        <div>
          <span className="text-accent-color">{card.total_decks}</span> decks (
          {Math.floor((card.total_decks / card.all_decks) * 100)}%)
        </div>
        <div>Rank #{card.rank}</div>
      </div>
      {children}
    </>
  )
}
