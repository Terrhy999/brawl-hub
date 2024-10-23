// app/api/search/[cardName]/route.ts

import { NextResponse } from 'next/server'

export async function GET({ params }: { params: { cardName: string } }) {
  const { cardName } = params

  try {
    // Fetch data from your Rust API
    const apiResponse = await fetch(`http://192.168.0.238:3030/search/${encodeURIComponent(cardName)}`)

    // Check if the response is ok
    if (!apiResponse.ok) {
      console.error('API error:', apiResponse.statusText) // Log the error
      return NextResponse.json({ error: 'Failed to fetch data' }, { status: 500 })
    }

    // Parse the response data
    const data = await apiResponse.json()

    // Return the data as a JSON response
    return NextResponse.json(data)
  } catch (error) {
    console.error('Fetch error:', error) // Log any errors
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 })
  }
}
