import { cookies } from 'next/headers';

export async function GET() {
  cookies().delete('id');
  return new Response(null, { status: 200 });
}