import CardGrid, { Card } from '../_components/card-grid'
import React from 'react'
import { fetchJson } from '../_utils/fetch-json'

async function getTopCards(): Promise<Card[]> {
  return await fetchJson<Card[]>('http://127.0.0.1:3030/top_cards', {
    cache: 'no-cache',
  })
}

export default async function Page() {
  const topCards = await getTopCards()
  return <CardGrid cards={topCards}>{CardText}</CardGrid>
}

function CardText(card: Card, i: number): React.ReactNode {
  return (
    <div className="text-center">
      <div>Rank #{i + 1}</div>
      <div>{card.count} decks</div>
    </div>
  )
}
