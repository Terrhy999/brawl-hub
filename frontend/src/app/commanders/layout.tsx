'use client'

import React from 'react'
import { usePathname } from 'next/navigation'
import Link from 'next/link'
import { colorCombinations } from '../_utils/color-combinations'
import { ColorIdentityFilter } from '../_components/color-identity-filter'

export default function Layout({ children }: { children: React.ReactNode }) {
  const pathname = usePathname()?.split('/')[2] ?? ''
  const colorCombinationName = colorCombinations.find((combo) => pathname === combo.colorIdentity)?.title ?? ''
  return (
    <main className="max-w-[85%] mx-auto">
      <h1 className="text-[32px]">Top {colorCombinationName} Commanders</h1>
      <ColorIdentityFilter />
      {children}
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
      className={`rounded-[8px] bg-white/[0.1] h-[32px] px-[12px] font-medium whitespace-nowrap ${activeClass} ${className}`}
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
