import API from '@/lib/api';
import auth from 'auth';

export default async function Home() {
  const user = await auth();
  if (!user) return 'Not signed in';
  const test = await API.get('profile/test');
  return (
    <>
      <pre>{JSON.stringify(test, null, 2)}</pre>
    </>
  );
}
