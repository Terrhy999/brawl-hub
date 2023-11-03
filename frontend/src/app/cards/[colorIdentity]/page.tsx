import CardGrid, { Card } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'
import { fetchJson } from '@/app/_utils/fetch-json'

export const dynamicParams = false

export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const cardsOfColorIdentity = await fetchJson<Card[]>(`http://127.0.0.1:3030/top_cards/${params.colorIdentity}`)
  return <CardGrid cards={cardsOfColorIdentity} />
}
