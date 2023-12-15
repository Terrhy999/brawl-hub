'use client'

import Image from 'next/image'
import { Card } from './card-grid'
import { useState } from 'react'
import Link from 'next/link'

export function FlipCard({
  card,
  className = '',
  size = 'normal',
}: {
  card: Card
  className?: string
  size?: 'normal' | 'large'
}) {
  const width = size === 'normal' ? 244 : 336
  const height = size === 'normal' ? 340 : 468
  let [isFlipped, setFlipped] = useState(false)
  return (
    <div className="relative">
      <Link href={`/commander/${card.slug}`}>
        <div
          className={`relative [transform-style:preserve-3d] [transition:transform_1s] ${
            isFlipped ? '[transform:rotateY(-180deg)]' : ''
          }`}
        >
          <div className={`[backface-visibility:hidden] relative z-10 [transform:rotateY(0deg)]`}>
            <Image
              className={`rounded-[5%] ${className}`}
              src={size === 'normal' ? card.image_normal : card.image_large}
              alt={card.name_full}
              width={width}
              height={height}
            />
          </div>
          <div className="absolute top-0 [backface-visibility:hidden] [transform:rotateY(180deg)]">
            <Image
              className={`rounded-[5%] ${className}`}
              src={size === 'normal' ? card?.image_normal_back ?? '' : card?.image_large_back ?? ''}
              alt={card.name_full}
              width={width}
              height={height}
              hidden={card?.image_normal_back == null}
            />
          </div>
        </div>
      </Link>
      <button
        className="absolute top-[30%] right-[9%] border-2 rounded-full bg-white opacity-70 border-black p-1"
        onClick={() => {
          setFlipped(!isFlipped)
        }}
      >
        <FlipSvg />
      </button>
    </div>
  )
}

function FlipSvg() {
  return (
    <svg width="25px" height="25px" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="none">
      <g fill="#000000">
        <path d="M8 1.5A6.5 6.5 0 0114.5 8 .75.75 0 0016 8 8 8 0 002.5 2.19v-.94a.75.75 0 00-1.5 0v3c0 .414.336.75.75.75h3a.75.75 0 000-1.5H3.31A6.479 6.479 0 018 1.5zM.75 7.25A.75.75 0 000 8a8 8 0 0013.5 5.81v.94a.75.75 0 001.5 0v-3a.75.75 0 00-.75-.75h-3a.75.75 0 000 1.5h1.44A6.5 6.5 0 011.5 8a.75.75 0 00-.75-.75z" />{' '}
      </g>
    </svg>
  )
}
