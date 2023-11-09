import { usePathname, useRouter } from 'next/navigation'
import { useEffect, useState } from 'react'
import { colorCombinations } from '../_utils/color-combinations'
import Image from 'next/image'

type Colors = 'w' | 'u' | 'b' | 'r' | 'g' | 'colorless'

const colors = [
  ['w', '/white-mana-symbol.png'],
  ['u', '/blue-mana-symbol.png'],
  ['b', '/black-mana-symbol.png'],
  ['r', '/red-mana-symbol.png'],
  ['g', '/green-mana-symbol.png'],
] as const

export function ColorIdentityFilter({ goTo = 'commanders' }: { goTo?: 'commanders' | 'cards' }) {
  const pathname = usePathname()?.split('/')[2] ?? ''
  let alreadySelectedColors: Set<Colors> | undefined = undefined
  if (pathname === 'colorless') {
    alreadySelectedColors = new Set<Colors>(['colorless'])
  } else {
    alreadySelectedColors = new Set([...(pathname.split('') as Colors[])])
  }
  const [selectedColors, setSelectedColors] = useState(new Set<Colors>(alreadySelectedColors))
  const router = useRouter()
  useEffect(() => {
    const colorIdentitys = colorCombinations.map((combo) => combo.colorIdentity)
    const sortedCompareString = [...selectedColors].join('').split('').sort().join('')
    let navigateTo =
      colorIdentitys.find((s) => {
        const sortedIdentitys = s.split('').sort().join('')
        return s.length === sortedCompareString.length && sortedIdentitys === sortedCompareString
      }) ?? ''
    router.push(`/${goTo}/${navigateTo}`)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedColors])
  return (
    <div className="flex justify-between mb-5">
      <div className="flex [&>*]:mr-[20px] [&>button]:opacity-30 [&>*]:duration-[0.3s]">
        {colors.map(([color, path], i) => {
          return (
            <button
              key={i}
              className={`${pathname.includes(color) && pathname != 'colorless' ? '!opacity-[unset]' : ''}`}
              onClick={() => {
                if (selectedColors.has(color)) {
                  selectedColors.delete(color)
                  setSelectedColors(new Set([...selectedColors]))
                } else {
                  selectedColors.delete('colorless')
                  setSelectedColors(new Set([...selectedColors, color]))
                }
              }}
            >
              <Image src={path} alt={'White mana'} width={36} height={36} />
            </button>
          )
        })}
        <button className={`${pathname.includes('colorless') ? '!opacity-[unset]' : ''}`} onClick={() => {}}>
          <Image
            onClick={() => {
              if (selectedColors.has('colorless')) {
                selectedColors.delete('colorless')
                setSelectedColors(new Set([...selectedColors]))
              } else {
                setSelectedColors(new Set(['colorless']))
                router.push(`${goTo}/colorless`)
              }
            }}
            src={'/colorless-mana.png'}
            alt={'Colorless Mana'}
            width={36}
            height={36}
          />
        </button>
        <button className={`${selectedColors.size > 0 ? '!opacity-[unset]' : ''}`} onClick={() => {}}>
          <Image
            onClick={() => {
              setSelectedColors(new Set())
              router.push(`/${goTo}/`)
            }}
            src={'/untap-symbol.svg'}
            alt={'Colorless Mana'}
            width={36}
            height={36}
          />
        </button>
      </div>
    </div>
  )
}
