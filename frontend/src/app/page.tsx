import Image from "next/image";

async function getTopCommanders() {
    const res = await fetch("http://127.0.0.1:3030/get_top_commanders", {
        // const res = await fetch("http://127.0.0.1:3030/get_top_commanders", {
        cache: "no-cache",
    });
    return res.json();
}

export type Card = {
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
    count?: number | null;
};

export default async function Home() {
    let activeDateFilter = 'year';
    const top_commanders: Card[] = await getTopCommanders();
    return (
        // <div className="bg-[#22262a] text-white">
        <main>
            <h1 className="text-[32px]">Top Commanders</h1>
            <div className="flex justify-between mb-5">
                {/* Change this to a radio button */}
                <span className="flex [&>*]:mr-[12px]">
                    <ClickableChip text={"Year"} isActive={activeDateFilter === "year"} />
                    <ClickableChip text={"Month"} isActive={activeDateFilter === "month"} />
                    <ClickableChip text={"Week"} isActive={activeDateFilter === "week"} />
                </span>

                <div className="flex [&>*]:mr-[20px] [&>*]:opacity-30 [&>*]:duration-[0.3s]">
                    <Image src={"/white-mana-symbol.png"} alt={"Green mana"} width={36} height={36} />
                    <Image src={"/blue-mana-symbol.png"} alt={"Green mana"} width={36} height={36} />
                    <Image src={"/black-mana-symbol.png"} alt={"Green mana"} width={36} height={36} />
                    <Image src={"/red-mana-symbol.png"} alt={"Green mana"} width={36} height={36} />
                    <Image src={"/green-mana-symbol.png"} alt={"Green mana"} width={36} height={36} />
                </div>
            </div>

            <div className="grid gap-[20px] grid-cols-[repeat(auto-fit,minmax(270px,1fr))]">
                {top_commanders.map((c, i: number) => (
                    <CardAndRank key={i} i={i} c={c} />
                ))}
            </div>
        </main>
    );
}

export function ClickableChip({
    text,
    isActive = false,
    onClick = undefined,
}: {
    text: string;
    isActive?: boolean;
    onClick?: () => void | undefined;
}) {
    const activeClass = isActive
        ? "!bg-[rgb(241,241,241)] text-[rgb(15,15,15)] "
        : "";
    return (
        <button
            onClick={onClick}
            className={`rounded-[8px] bg-white/[0.1] h-[32px] w-m-[12px] px-[12px] font-medium flex items-center ${activeClass}`}
        >
            {text}
        </button>
    );
}

function CardAndRank({ i, c }: { i: number; c: Card }) {
    return (
        <div className="flex flex-col items-center">
            <Image
                className="rounded-[5%] max-h-[340px]"
                src={c.image_large}
                alt={c.name}
                width={244}
                height={340}
            />
            <div className="text-center">
                <div>Rank #{i + 1}</div>
                <div>{c.count} decks</div>
            </div>
        </div>
    );
}
