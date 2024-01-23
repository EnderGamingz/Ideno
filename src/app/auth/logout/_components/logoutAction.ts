'use server';
import API from '@/lib/api';
import { cookies } from 'next/headers';

export default async function LogoutAction() {
  await API.auth
    .logout()
    .then(() => {
      cookies().delete('id');
      return { success: true };
    })
    .catch(() => {});
  return { noEffect: true };
}
