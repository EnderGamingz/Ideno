'use server';
import API from '@/lib/api';
import { cookies } from 'next/headers';
import { redirect } from 'next/navigation';

export default async function LogoutAction() {
  await API.auth
    .logout()
    .then(() => {
      cookies().delete('id');
    })
    .catch(() => {});
  redirect('/');
}
