import CardGrid, { Card } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'

export const dynamicParams = false

export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

async function getCommandersByColorIdentity(colorIdentity: string): Promise<Card[]> {
  const commandersOfColors: Card[] = await fetch(
    `http://127.0.0.1:3030/top_cards_for_color_identity/${colorIdentity}`
  ).then((res) => res.json())
  return commandersOfColors
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const commanders = await getCommandersByColorIdentity(params.colorIdentity)
  return <CardGrid cards={commanders} />
}
