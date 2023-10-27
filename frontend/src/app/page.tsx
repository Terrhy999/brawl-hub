async function getTopCommanders() {
  const res = await fetch("http://localhost:3030/get_top_commanders", {
    cache: "no-cache",
  });
  return res.json();
}

async function getTopCards() {
  const res = await fetch("http://localhost:3030/get_top_cards", {
    cache: "no-cache",
  });
  return res.json();
}

type CommanderInfo = {
  oracle_id: string; // Assuming Uuid is represented as a string
  name: string;
  lang: string;
  scryfall_uri: string;
  layout: string;
  mana_cost: string | null;
  cmc: number;
  type_line: string;
  oracle_text: string | null;
  colors: (string | null)[];
  color_identity: string[];
  is_legal: boolean;
  is_commander: boolean;
  rarity: string;
  image_small: string;
  image_normal: string;
  image_large: string;
  image_art_crop: string;
  image_border_crop: string;
  count: number;
};

type TopCard = {
  oracle_id: string; // Assuming Uuid is represented as a string
  name: string;
  lang: string;
  scryfall_uri: string;
  layout: string;
  mana_cost: string | null;
  cmc: number;
  type_line: string;
  oracle_text: string | null;
  colors: (string | null)[];
  color_identity: string[];
  is_legal: boolean;
  is_commander: boolean;
  rarity: string;
  image_small: string;
  image_normal: string;
  image_large: string;
  image_art_crop: string;
  image_border_crop: string;
  number_of_decks: number;
};

export default async function Home() {
  const top_commanders: CommanderInfo[] = await getTopCommanders();
  const top_cards: TopCard[] = await getTopCards();
  console.log(top_commanders);
  return (
    <main className="flex flex-row">
      <div>
        {top_commanders.map((c, i: number) => (
          <div key={i}>
            {c.count} of {c.name}
            <img src={c.image_small} alt={c.name}></img>
          </div>
        ))}
      </div>
      <div>
        {top_cards.map((c, i: number) => (
          <div key={i}>
            {c.number_of_decks} of {c.name}
            <img src={c.image_small} alt={c.name}></img>
          </div>
        ))}
      </div>
    </main>
  );
}
