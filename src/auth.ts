'use server';
import { cookies } from 'next/headers';
import API from '@/lib/api';

export default async function auth() {
  const token = cookies().get('id')?.value;
  if (!token) return undefined;

  try {
    return await API.auth();
  } catch (e) {
    return undefined;
  }
}
