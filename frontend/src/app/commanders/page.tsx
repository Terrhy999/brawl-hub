import Image from 'next/image'
import { Card } from '../page'

async function getTopCommanders() {
  const res = await fetch('http://127.0.0.1:3030/get_top_commanders', {
    // const res = await fetch("http://127.0.0.1:3030/get_top_commanders", {
    cache: 'no-cache',
  })
  return res.json()
}

export default async function Page() {
  const top_commanders: Card[] = await getTopCommanders()
  return (
    <div className="grid gap-[20px] grid-cols-[repeat(auto-fit,minmax(270px,1fr))]">
      {top_commanders.map((c, i: number) => (
        <CardAndRank key={i} i={i} c={c} />
      ))}
    </div>
  )
}

function CardAndRank({ i, c }: { i: number; c: Card }) {
  return (
    <div className="flex flex-col items-center">
      <Image className="rounded-[5%] max-h-[340px]" src={c.image_large} alt={c.name} width={244} height={340} />
      <div className="text-center">
        <div>Rank #{i + 1}</div>
        <div>{c.count} decks</div>
      </div>
    </div>
  )
}
