import { cookies } from 'next/headers';
import { redirect } from 'next/navigation';

export async function clearSession() {
  cookies().delete('id');
}

export async function GET() {
  await clearSession();
  redirect('/auth/login');

  return new Response('Invalid session cleared', {
    status: 200,
  });
}
