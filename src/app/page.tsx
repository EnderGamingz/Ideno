import API from '@/lib/api';
import auth from 'auth';

export default async function Home() {
  const user = await auth();
  if (!user) return <h1>Not signed in</h1>;
  const test = await API.get('profile/test').then(res => res.data);
  return (
    <>
      <h1>Test</h1>
      <pre>{JSON.stringify(test, null, 2)}</pre>
    </>
  );
}
