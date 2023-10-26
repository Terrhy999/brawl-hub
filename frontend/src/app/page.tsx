async function getTopCommanders() {
  const res = await fetch("http://localhost:3030");
  return res.json();
}

export default async function Home() {
  const top_commanders = await getTopCommanders();
  console.log(top_commanders);
  return (
    <main>
      {top_commanders.map(
        (c: { commander: string; count: number }, i: number) => (
          <div key={i}>
            {c.count} of {c.commander}
          </div>
        )
      )}
    </main>
  );
}
