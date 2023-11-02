import CardGrid, { Card } from '../_components/card-grid'
import React from 'react'

async function getTopCards(): Promise<Card[]> {
  const res = await fetch('http://127.0.0.1:3030/top_cards', {
    cache: 'no-cache',
  })
  return res.json()
}

export default async function Page() {
  const topCards = await getTopCards()
  return <CardGrid cards={topCards}>{cardText}</CardGrid>
}

function cardText(card: Card, i: number): React.ReactNode {
  return (
    <div className="text-center">
      <div>Rank #{i + 1}</div>
      <div>{card.count} decks</div>
    </div>
  )
}
