import auth from 'auth';
import API from '@/lib/api';

export default async function Home() {
  const user = await auth();
  if (!user) return <h1>Not signed in</h1>;
  const profile = await API.auth.profile.get();
  return (
    <>
      <h1>Test</h1>
      <pre>{JSON.stringify(user, null, 2)}</pre>
      <pre>{JSON.stringify(profile, null, 2)}</pre>
    </>
  );
}
