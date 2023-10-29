import { Card } from '@/app/_components/commander-card-gallery'

export const dynamicParams = false

export async function generateStaticParams() {
  function santisizeName(name: string): string {
    return name.replaceAll(',', '').replaceAll(' ', '-').toLocaleLowerCase()
  }
  const commandersName = (await getAllCommanders()).map((commander) => ({
    commander: santisizeName(commander.name),
  }))
  return commandersName
}

async function getAllCommanders(): Promise<Card[]> {
  return await fetch(`http://127.0.0.1:3030/all_commanders`).then((res) => res.json())
}

export default async function Page({ params }: { params: { commander: string } }) {
  console.log(params)
  return <div>{params.commander}</div>
}
