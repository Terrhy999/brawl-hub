import { Card, TopCard } from './card-grid'

export function CardPage({ card, children }: { card: TopCard; children: React.ReactNode }) {
  return (
    <>
      <div
        style={{ ['--image-url' as any]: `url(${card.image_art_crop})` }}
        className={`absolute blur-md bg-cover w-full h-[442px] bg-[image:var(--image-url)]`}
      ></div>
      <div className="text-center z-40 relative flex flex-col items-center">
        {/* <h2 className="text-4xl">{card.name}</h2> */}
        <Card className="mx-auto my-2" card={card} size="large" />
        {/* <div>
          {Math.floor((card.total_decks / card.total_commander_decks_of_ci) * 100)}% decks with this Color Identity
        </div> */}
        <div>
          In {card.total_decks_with_card} of {card.total_decks_could_play} decks (
          {Math.round((card.total_decks_with_card! * 10000) / card.total_decks_could_play!) / 100}%)
        </div>
      </div>
      {children}
    </>
  )
}
