import { cookies } from 'next/headers';
import { redirect } from 'next/navigation';

export async function GET() {
  cookies().delete('id');

  redirect('/auth/login');

  return new Response('Invalid session cleared', {
    status: 200,
  });
}
