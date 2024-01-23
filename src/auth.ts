'use server';
import { cookies } from 'next/headers';
import API from '@/lib/api';
import { redirect } from 'next/navigation';

export default async function auth() {
  const token = cookies().get('id')?.value;
  if (!token) return undefined;
  let user;
  try {
    user = await API.auth.auth();
  } catch (e) {
    console.error(e);
    return undefined;
  }

  if (user === undefined) {
    console.log('Invalid session found. Trying to clear');
    redirect('/api/session/clear');
  }
  return user;
}
