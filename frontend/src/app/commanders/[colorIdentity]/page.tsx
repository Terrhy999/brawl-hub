import CardGrid, { Card } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'

export const dynamicParams = false
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
  const commanders = await getCommandersByColorIdentity(params.colorIdentity)
  return (
    <CardGrid cards={commanders} linkTo="commander">
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
