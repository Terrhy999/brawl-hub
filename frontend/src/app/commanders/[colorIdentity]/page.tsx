import CardGrid, { Card } from '@/app/_components/card-grid'

export const dynamicParams = false
export const colorCombinations = [
  { colorIdentity: 'w', title: 'Mono-White' },
  { colorIdentity: 'u', title: 'Mono-Blue' },
  { colorIdentity: 'b', title: 'Mono-Black' },
  { colorIdentity: 'r', title: 'Mono-Red' },
  { colorIdentity: 'g', title: 'Mono-Green' },
  { colorIdentity: 'wu', title: 'Azorious' },
  { colorIdentity: 'ub', title: 'Dimir' },
  { colorIdentity: 'br', title: 'Rakdos' },
  { colorIdentity: 'rg', title: 'Gruul' },
  { colorIdentity: 'gw', title: 'Selesnya' },
  { colorIdentity: 'wb', title: 'Orzhov' },
  { colorIdentity: 'ur', title: 'Izzet' },
  { colorIdentity: 'bg', title: 'Golgari' },
  { colorIdentity: 'rw', title: 'Boros' },
  { colorIdentity: 'gu', title: 'Simic' },
  { colorIdentity: 'wub', title: 'Esper' },
  { colorIdentity: 'ubr', title: 'Grixis' },
  { colorIdentity: 'brg', title: 'Jund' },
  { colorIdentity: 'rgw', title: 'Naya' },
  { colorIdentity: 'gwu', title: 'Bant' },
  { colorIdentity: 'wbg', title: 'Abzan' },
  { colorIdentity: 'urw', title: 'Jeskai' },
  { colorIdentity: 'bgu', title: 'Sultai' },
  { colorIdentity: 'rwb', title: 'Mardu' },
  { colorIdentity: 'gur', title: 'Temur' },
  { colorIdentity: 'wubr', title: 'Sans-Green' },
  { colorIdentity: 'ubrg', title: 'Sans-White' },
  { colorIdentity: 'brgw', title: 'Sans-Blue' },
  { colorIdentity: 'rgwu', title: 'Sans-Black' },
  { colorIdentity: 'gwub', title: 'Sans-Red' },
  { colorIdentity: 'wubrg', title: 'Five-Color' },
  { colorIdentity: 'colorless', title: 'Colorless' },
] as const

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
  return <CardGrid cards={commanders} />
}
