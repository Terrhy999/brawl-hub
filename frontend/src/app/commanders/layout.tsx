'use client'

import Image from 'next/image'
import React, { useEffect, useState } from 'react'
import { colorCombinations } from './[colorIdentity]/page'
import { usePathname } from 'next/navigation'
import { useRouter } from 'next/navigation'
import Link from 'next/link'

type Colors = 'w' | 'u' | 'b' | 'r' | 'g' | 'colorless'

export default function ColorIdentityFilter({ children }: { children: React.ReactNode }) {
  let activeDateFilter = 'year'
  const pathname = usePathname()?.split('/')[2] ?? ''
  let alreadySelectedColors: Set<Colors> | undefined = undefined
  if (pathname === 'colorless') {
    alreadySelectedColors = new Set<Colors>(['colorless'])
  } else {
    alreadySelectedColors = new Set([...(pathname.split('') as Colors[])])
  }
  const [selectedColors, setSelectedColors] = useState(new Set<Colors>(alreadySelectedColors))
  const router = useRouter()
  const colorCombinationName = colorCombinations.find((combo) => pathname === combo.colorIdentity)?.title ?? ''
  useEffect(() => {
    const colorIdentitys = colorCombinations.map((combo) => combo.colorIdentity)
    const sortedCompareString = [...selectedColors].join('').split('').sort().join('')
    let navigateTo =
      colorIdentitys.find((s) => {
        const sortedIdentitys = s.split('').sort().join('')
        return s.length === sortedCompareString.length && sortedIdentitys === sortedCompareString
      }) ?? ''
    router.push(`/commanders/${navigateTo}`)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedColors])

  const colors = [
    ['w', '/white-mana-symbol.png'],
    ['u', '/blue-mana-symbol.png'],
    ['b', '/black-mana-symbol.png'],
    ['r', '/red-mana-symbol.png'],
    ['g', '/green-mana-symbol.png'],
  ] as const
  return (
    <main className="max-w-[85%] mx-auto">
      {/* <> */}
      <h1 className="text-[32px]">Top {colorCombinationName} Commanders</h1>
      <div className="flex justify-between mb-5">
        {/* Change this to a radio button */}
        {/* <span className="flex [&>*]:mr-[12px]">
          <ClickableChip text={'Year'} isActive={activeDateFilter === 'year'} />
          <ClickableChip text={'Month'} isActive={activeDateFilter === 'month'} />
          <ClickableChip text={'Week'} isActive={activeDateFilter === 'week'} />
        </span> */}

        <div className="flex [&>*]:mr-[20px] [&>button]:opacity-30 [&>*]:duration-[0.3s]">
          <button className={`${selectedColors.size > 0 ? '!opacity-[unset]' : ''}`} onClick={() => {}}>
            <Image
              onClick={() => {
                setSelectedColors(new Set())
                router.push('/commanders/')
              }}
              src={'/untap-symbol.svg'}
              alt={'Colorless Mana'}
              width={36}
              height={36}
            />
          </button>
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
                  router.push('/commanders/colorless')
                }
              }}
              src={'/colorless-mana.png'}
              alt={'Colorless Mana'}
              width={36}
              height={36}
            />
          </button>
        </div>
      </div>
      {children}
      {/* </> */}
    </main>
  )
}

export function ClickableChip({
  text,
  isActive = false,
  href = '',
  onClick = undefined,
  className = '',
}: {
  text: string
  isActive?: boolean
  href?: string
  onClick?: () => void | undefined
  className?: string
}) {
  const activeClass = isActive ? '!bg-[rgb(241,241,241)] text-[rgb(15,15,15)] ' : ''
  const button = (
    <button
      onClick={onClick}
      className={`rounded-[8px] bg-white/[0.1] h-[32px] w-m-[12px] px-[12px] font-medium ${activeClass} ${className}`}
    >
      {text}
    </button>
  )

  if (href) {
    return (
      <Link href={href} className={`${className}`}>
        {button}
      </Link>
    )
  }

  return button
}
