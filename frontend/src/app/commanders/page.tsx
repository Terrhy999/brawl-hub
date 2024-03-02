import React from 'react'
import CardGrid, { CardCount } from '../_components/card-grid'
import { fetchJsonFromBrawlhub } from '../_utils/fetch-json'

export default async function Page() {
  const topCommanders = await fetchJsonFromBrawlhub<CardCount[]>('commanders/')
  return (
    <CardGrid cards={topCommanders} linkTo="commander">
      {CardText}
    </CardGrid>
  )
}

function CardText(card: CardCount) {
  return (
    <div className="text-center mt-1">
      <div>{card.count} decks</div>
    </div>
  )
}
