import React from 'react'
import CardGrid, { Card } from '../_components/card-grid'
import { fetchJsonFromBrawlhub } from '../_utils/fetch-json'

export default async function Page() {
  const topCommanders = await fetchJsonFromBrawlhub<Card[]>('commanders/')

  return (
    <CardGrid cards={topCommanders} linkTo="commander">
      {CardText}
    </CardGrid>
  )
}

function CardText(card: Card, i: number): React.ReactNode {
  return (
    <div className="text-center">
      <div>Rank #{i + 1}</div>
      <div>{card.count} decks</div>
    </div>
  )
}
