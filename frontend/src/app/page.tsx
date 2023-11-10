import Link from 'next/link'

export default async function Home() {
  return (
    <div className="px-[5%] flex justify-center items-center h-screen flex-col [background:linear-gradient(to_top,#213B20,#1B341F,#162F1E,#132B1E,#10281D,#0A211C)]">
      <h1 className="text-[24px] md:text-[32px] text-center mb-[20px]">
        BrawlHub is the #1 spot to get rec&apos;s for your Historic Brawl decks!
      </h1>
      <input className="[border:1px_solid_rgba(255,255,255,0.25)] bg-[#0A211C] py-[12px] pr-[14px] pl-[14px] w-full max-w-[524px] focus:outline-none"></input>
      <nav className={`flex justify-between pt-[12px] mb-[200px]`}>
        <Link
          href={'/commanders/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)] mr-2"
        >
          Top Commanders
        </Link>
        <Link
          href={'/cards/'}
          className="[border:1px_solid_rgba(255,255,255,0.1)] px-[10px] py-[1px] rounded-[4px] bg-[color:rgba(255,255,255,.1)]"
        >
          Top Cards
        </Link>
      </nav>
    </div>
  )
}
