import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'
import { Card, CardCount } from '@/app/_components/card-grid'

export type Deck = {
  deck_id: number
  url: string
  username: string
  date_created: number
  date_updated: number
  commander: Card
  companion?: Card | null
  color_identity: string[]
  decklist: DeckList
}
type Sections =
  | 'newCards'
  | 'topCards'
  | 'creatures'
  | 'instants'
  | 'sorceries'
  | 'artifacts'
  | 'enchantments'
  | 'planeswalkers'
  | 'lands'
type DeckList = Record<Sections, CardCount[]>

async function getDecklist(deck_id: string): Promise<Deck> {
  return await fetchJsonFromBrawlhub<Deck>(`deck/${deck_id}`)
}

export default async function Page({ params }: { params: { deck: string } }) {
  let deck = await getDecklist(params.deck)
  let card = deck.commander

  const sections = [
    // ['Top Cards', ],
    // (TODO) replace the _ with a -
    ['Creatures', 'creatures', 'creature'],
    ['Instants', 'instants', 'instant'],
    ['Sorceries', 'sorceries', 'sorcery'],
    ['Artifacts', 'artifacts', 'artifact'],
    ['Enchantments', 'enchantments', 'enchantment'],
    ['Planeswalkers', 'planeswalkers', 'planeswalker'],
    ['Lands', 'lands', 'land'],
  ] as const

  return (
    <>
      <div
        style={{ ['--image-url' as any]: `url(${card.image_art_crop})` }}
        className={`absolute blur-md bg-cover w-full h-[442px] bg-[image:var(--image-url)]`}
      ></div>

      <div className="text-center z-40 relative flex flex-col items-center">
        <Card className="mx-auto my-2" card={card} size="large" />
      </div>
      <div className="max-w-[85%] mx-auto">
        <div className="columns-sm">
          {sections.map(([title, prop, symbol], i) => (
            <div key={i} className="mx-2 mb-6 min-w-[340px] w-auto h-min break-inside-avoid flex-grow">
              <div className="font-bold text-lg border-b border-gray-700/50 flex">
                <div className="me-2">
                  <i className={`ms ms-${symbol}`}></i>
                </div>
                {title} ({deck.decklist?.[prop]?.length ?? 0})
              </div>
              {deck.decklist?.[prop]?.map((card, i) => (
                <div key={i} className="border-b border-gray-700/50 flex justify-between">
                  <span className="ps-4 text-lg">
                    {card.count} {card.name_front}
                  </span>
                  <ManaCostRenderer manaCost={card.mana_cost_front!} />
                </div>
              ))}
            </div>
          ))}
        </div>
      </div>
    </>
  )
}

const ManaCostRenderer = ({ manaCost }: { manaCost: string }) => {
  const renderManaSymbols = () => {
    const regex = /{([^}]+)}/g
    const matches = manaCost.match(regex)

    if (!matches) {
      return null
    }

    return matches.map((symbol, index) => (
      <i key={index} className={`mx-px ms ms-cost ms-${symbol.replace(/[{}]/g, '').toLowerCase()}`}></i>
    ))
  }

  return <div className="mana-cost">{renderManaSymbols()}</div>
}
