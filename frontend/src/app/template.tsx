'use client'
import { NavBar } from './_components/navbar'

export default function Template({ children }: { children: React.ReactNode }) {
  return (
    <>
      <NavBar />
      <div>{children}</div>
    </>
  )
}
