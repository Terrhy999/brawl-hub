import { Card, ClickableChip } from "@/app/page";
import Image from "next/image";

export const dynamicParams = false;
export async function generateStaticParams() {
    const commanderColors = ["w", "u", "b", "r", "g", "colorless"];
    return commanderColors.map((colorIdentity) => ({ colorIdentity }));
}

async function getCommandersByColorIdentity(
    colorIdentity: string
): Promise<Card[]> {
    const commandersOfColors: Card[] = await fetch(
        `http://127.0.0.1:3030/commanders/${colorIdentity}`
    ).then((res) => res.json());
    return commandersOfColors;
}

export default async function Page({ params }: { params: { colorIdentity: string } }) {
    console.log("type of window: ", typeof window)
    console.log(params);
    let activeDateFilter = 'year';
    const top_commanders = await getCommandersByColorIdentity(params.colorIdentity);
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
                {top_commanders.map((card, i: number) => (
                    <CardAndRank key={i} i={i} card={card} />
                ))}
            </div>
        </main>
    );
}

// replace 'i' with count
function CardAndRank({ i, card }: { i: number; card: Card }) {
    return (
        <div className="flex flex-col items-center">
            <Image
                className="rounded-[5%] max-h-[340px]"
                src={card.image_large}
                alt={card.name}
                width={244}
                height={340}
            />
            <div className="text-center">
                <div>Rank #{i + 1}</div>
                <div>{card.count} decks</div>
            </div>
        </div>
    );
}
