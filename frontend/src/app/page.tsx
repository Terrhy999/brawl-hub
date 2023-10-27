import Image from "next/image";

async function getTopCommanders() {
    const res = await fetch("http://127.0.0.1:3030/get_top_commanders", {
        cache: "no-cache",
    });
    return res.json();
}

async function getTopCards() {
    const res = await fetch("http://127.0.0.1:3030/get_top_cards", {
        cache: "no-cache",
    });
    return res.json();
}

type Card = {
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
    const top_commanders: Card[] = await getTopCommanders();
    // const top_cards: TopCard[] = await getTopCards();
    // console.log(top_commanders);
    return (
        <div className="bg-[#22262a] text-white">
            <NavBar />
            <main className="m-10">
                <h1 className="text-[32px]">Top Commanders</h1>
                <div className="grid gap-[20px] grid-cols-[repeat(auto-fit,minmax(270px,1fr))]">
                    {top_commanders.map((c, i: number) => (
                        <CardAndRank key={i} i={i} c={c} />
                    ))}
                </div>
            </main>
        </div>
    );
}

function NavBar() {
    return (
        <header className="h-[56px] bg-[#a2ac94] flex items-center p-4">
            BrawlRec
            <nav></nav>
        </header>
    )
}

function CardAndRank({i, c} : {i: number, c: Card}) {
    return (
        <div className="flex flex-col items-center">
            <Image className="rounded-[5%] max-h-[340px]" src={c.image_large} alt={c.name} width={244} height={340} />
            <div className="">
                <div>Rank #{i + 1}</div>
                <div>{c.count} decks</div>
            </div>
        </div>
    );
}
