'use server';

import { z } from 'zod';
import API from '@/lib/api';
import { cookies } from 'next/headers';

const schema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

export default async function loginSubmit(_: any, formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }
  try {
    const response = await API.auth.login(
      parsed.data.username,
      parsed.data.password,
    );
    const json = await response.json().catch(() => {
      return { message: 'An unexpected error has occurred.' };
    });

    if (!response.ok) {
      return { error: json.message || response.statusText };
    }
    let cookieValues = response.headers.getSetCookie();
    for (const cookie of cookieValues) {
      const parsedCookie = cookie.split(';')[0];
      cookies().set(parsedCookie.split('=')[0], parsedCookie.split('=')[1]);
    }
  } catch (err) {
    console.log(err);
    return { error: 'Something went wrong' };
  }
  return { success: true };
}
