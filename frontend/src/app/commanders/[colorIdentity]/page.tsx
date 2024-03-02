import CardGrid, { CardCount } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'
import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'

export const dynamicParams = false
export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const commandersOfColorIdentity = await fetchJsonFromBrawlhub<CardCount[]>(`commanders/${params.colorIdentity}`)
  return (
    <CardGrid cards={commandersOfColorIdentity} linkTo="commander">
      {CardText}
    </CardGrid>
  )
}

function CardText(card: CardCount) {
  return (
    <div className="text-center mt-1">
      <div>{card.count} decks</div>
    </div>
  )
}
