import CardGrid, { Card } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'
import { fetchJson } from '@/app/_utils/fetch-json'

export const dynamicParams = false
export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const commandersOfColorIdentity = await fetchJson<Card[]>(`http://127.0.0.1:3030/commanders/${params.colorIdentity}`)
  return (
    <CardGrid cards={commandersOfColorIdentity} linkTo="commander">
      {CardText}
    </CardGrid>
  )
}

function CardText(card: Card, i: number): React.ReactNode {
  return (
    <div className="text-center">
      <div>Rank #{i + 1}</div>
      <div>{card.count} decks</div>
    </div>
  )
}
