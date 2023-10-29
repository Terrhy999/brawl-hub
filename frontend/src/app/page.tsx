export type Card = {
  oracle_id: string // Assuming Uuid is represented as a string
  name: string
  lang: string
  scryfall_uri: string
  layout: string
  mana_cost: string | null
  cmc: number
  type_line: string
  oracle_text: string | null
  colors: (string | null)[]
  color_identity: string[]
  is_legal: boolean
  is_commander: boolean
  rarity: string
  image_small: string
  image_normal: string
  image_large: string
  image_art_crop: string
  image_border_crop: string
  count?: number | null
}

export default async function Home() {
  return <main></main>
}
