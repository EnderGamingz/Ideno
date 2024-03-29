import { redirect } from 'next/navigation';
import { clearSession } from '@/app/api/session/clear/clearSession';

export async function GET() {
  await clearSession();
  redirect('/auth/login');

  return new Response('Invalid session cleared', {
    status: 200,
  });
}
