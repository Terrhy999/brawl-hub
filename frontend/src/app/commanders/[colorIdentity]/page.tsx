import { Card, CardGridWithText } from '@/app/_components/card-grid'
import { colorCombinations } from '@/app/_utils/color-combinations'
import { fetchJsonFromBrawlhub } from '@/app/_utils/fetch-json'

export const dynamicParams = false
export async function generateStaticParams() {
  return colorCombinations.map(({ colorIdentity }) => ({ colorIdentity }))
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
  const commandersOfColorIdentity = await fetchJsonFromBrawlhub<Card[]>(`commanders/${params.colorIdentity}`)
  return <CardGridWithText cards={commandersOfColorIdentity} linkTo="commander" />
}
