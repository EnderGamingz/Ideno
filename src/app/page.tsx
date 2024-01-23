import auth from 'auth';

export default async function Home() {
  const user = await auth();
  if (!user) return <h1>Not signed in</h1>;
  return (
    <>
      <h1>Test</h1>
      <pre>{JSON.stringify(user, null, 2)}</pre>
    </>
  );
}
