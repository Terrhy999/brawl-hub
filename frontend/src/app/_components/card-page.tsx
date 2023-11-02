import { Card } from './card-grid'

export function CardPage({ card, children }: { card: Card; children: React.ReactNode }) {
  return (
    <>
      <div
        style={{ ['--image-url' as any]: `url(${card.image_art_crop})` }}
        className={`absolute blur-md bg-cover w-full h-[442px] bg-[image:var(--image-url)]`}
      ></div>
      <div className="text-center z-40 relative">
        <h2 className="text-4xl">{card.name}</h2>
        <Card className="mx-auto my-2" card={card} size="large" />
        <div>
          <span className="text-accent-color">{card.count}</span> decks (%) of Decks with this Color Identity
        </div>
        <div>
          <span className="text-accent-color">{card.count}</span> decks (%)
        </div>
        <div>Rank #</div>
      </div>
      {children}
    </>
  )
}
