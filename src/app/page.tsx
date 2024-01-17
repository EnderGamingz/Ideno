import API from '@/lib/api';
import auth from 'auth';
import { NestedAuthenticationTest } from '@/app/nestedAuthenticationTest';

export default async function Home() {
  const user = await auth();
  if (!user) return 'Not signed in';
  console.log('user authenticated');
  const test = await API.get('auth/profile/contact-information');
  return (
    <>
      <pre>{JSON.stringify(test, null, 2)}</pre>
      <NestedAuthenticationTest />
    </>
  );
}
