import CommanderCardGallery from '../_components/commander-card-gallery'

async function getTopCommanders() {
  const res = await fetch('http://127.0.0.1:3030/get_top_commanders', {
    cache: 'no-cache',
  })
  return res.json()
}

export default async function Page() {
  const top_commanders = await getTopCommanders()
  return <CommanderCardGallery commanders={top_commanders} />
}
