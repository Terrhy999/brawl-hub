import CardGrid from '../_components/card-grid'

async function getTopCommanders() {
  const res = await fetch('http://127.0.0.1:3030/commanders/', {
    cache: 'no-cache',
  })
  return res.json()
}

export default async function Page() {
  const topCommanders = await getTopCommanders()
  return <CardGrid cards={topCommanders} linkTo="commander" />
}
