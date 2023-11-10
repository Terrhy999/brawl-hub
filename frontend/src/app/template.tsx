'use client'
import { usePathname } from 'next/navigation'
import { NavBar } from './_components/navbar'

export default function Template({ children }: { children: React.ReactNode }) {
  return (
    <>
      {usePathname() === '/' ? undefined : <NavBar />}
      {children}
    </>
  )
}
