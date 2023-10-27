async function getTopCommanders() {
  const res = await fetch("http://localhost:3030", { cache: "no-cache" });
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

export default async function Home() {
  const top_commanders: CommanderInfo[] = await getTopCommanders();
  console.log(top_commanders);
  return (
    <main>
      {top_commanders.map((c, i: number) => (
        <div key={i}>
          {c.count} of {c.name}
          <img src={c.image_small} alt={c.name}></img>
        </div>
      ))}
    </main>
  );
}
