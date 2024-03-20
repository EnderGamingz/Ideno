'use server';
import { cookies } from 'next/headers';
import API from '@/lib/api';
import { redirect } from 'next/navigation';
import { UserModel } from '@/types/user';

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

  if (!user) {
    console.log('Invalid session found. Trying to clear');
    redirect('/api/session/clear');
  }
  return user;
}

export async function admin(): Promise<UserModel | undefined> {
  const user = (await auth()) as UserModel | undefined;
  if (!user) redirect('/auth/login');
  if (user?.role !== 'admin') return undefined;
  return user;
}
