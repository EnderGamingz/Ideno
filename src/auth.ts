'use server';
import { cookies } from 'next/headers';
import API from '@/lib/api';

export default async function auth() {
  try {
    const token = cookies().get('id')?.value;
    if (!token) return undefined;

    return await API.auth.auth();
  } catch (e) {
    return undefined;
  }
}
