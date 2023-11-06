import CardGrid, { Card } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'
import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'

export const dynamicParams = false

export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const cardsOfColorIdentity = await fetchJsonFromBrawlhub<Card[]>(`top_cards/${params.colorIdentity}`)
  return <CardGrid cards={cardsOfColorIdentity} />
}
