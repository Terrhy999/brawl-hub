'use client'

import Image from 'next/image'
import React, { useEffect, useState } from 'react'
import { colorCombinations } from './[colorIdentity]/page'
import { usePathname } from 'next/navigation'
import { useRouter } from 'next/navigation'
import { ClickableChip } from '../page'

type Colors = 'w' | 'u' | 'b' | 'r' | 'g'

export default function ColorIdentityFilter({ children }: { children: React.ReactNode }) {
  let activeDateFilter = 'year'
  const [selectedColors, setSelectedColors] = useState(new Set<Colors>())
  const router = useRouter()
  const pathname = usePathname()?.split('/')[2] ?? ''
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
    <main>
      <h1 className="text-[32px]">Top {colorCombinationName} Commanders</h1>
      <div className="flex justify-between mb-5">
        {/* Change this to a radio button */}
        <span className="flex [&>*]:mr-[12px]">
          <ClickableChip text={'Year'} isActive={activeDateFilter === 'year'} />
          <ClickableChip text={'Month'} isActive={activeDateFilter === 'month'} />
          <ClickableChip text={'Week'} isActive={activeDateFilter === 'week'} />
        </span>

        <div className="flex [&>*]:mr-[20px] [&>button]:opacity-30 [&>*]:duration-[0.3s]">
          {colors.map(([color, path], i) => {
            return (
              <button
                key={i}
                className={`${pathname.includes(color) ? '!opacity-[unset]' : ''}`}
                onClick={() => {
                  if (selectedColors.has(color)) {
                    selectedColors.delete(color)
                    setSelectedColors(new Set([...selectedColors]))
                  } else {
                    setSelectedColors(new Set([...selectedColors, color]))
                  }
                }}
              >
                <Image src={path} alt={'White mana'} width={36} height={36} />
              </button>
            )
          })}
        </div>
      </div>
      {children}
    </main>
  )
}
