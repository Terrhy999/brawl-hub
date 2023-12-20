import Link from 'next/link'

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
