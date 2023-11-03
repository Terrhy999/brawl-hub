'use client'

import React from 'react'
import { usePathname } from 'next/navigation'
import { colorCombinations } from '../_utils/color-combinations'
import { ColorIdentityFilter } from '../_components/color-identity-filter'

export default function Layout({ children }: { children: React.ReactNode }) {
  const pathname = usePathname()?.split('/')[2] ?? ''
  const colorCombinationName = colorCombinations.find((combo) => pathname === combo.colorIdentity)?.title ?? ''
  return (
    <main className="max-w-[85%] mx-auto">
      <h1 className="text-[32px]">Top {colorCombinationName} Cards</h1>
      <ColorIdentityFilter goTo="cards" />
      {children}
    </main>
  )
}
