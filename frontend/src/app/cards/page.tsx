import { TopCard, CardGridWithText } from '../_components/card-grid'
import React from 'react'
import { fetchJsonFromBrawlhub } from '../_utils/fetch-json'

export default async function Page() {
  const topCards = await fetchJsonFromBrawlhub<TopCard[]>('top_cards')
  return <CardGridWithText cards={topCards} />
}
