import React from 'react'
import { Card, CardGridWithText } from '../_components/card-grid'
import { fetchJsonFromBrawlhub } from '../_utils/fetch-json'

export default async function Page() {
  const topCommanders = await fetchJsonFromBrawlhub<Card[]>('commanders/')
  return <CardGridWithText cards={topCommanders} linkTo="commander" />
}
