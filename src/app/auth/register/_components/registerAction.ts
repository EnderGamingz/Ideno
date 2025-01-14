'use server';

import { z } from 'zod';
import API from '@/lib/api';
import { redirect } from 'next/navigation';

const schema = z.object({
  username: z.string().min(1),
  email: z.string().email().min(1).email(),
  password: z.string().min(1),
});

export default async function registerSubmit(_: any, formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }
  try {
    const response = await API.auth.register(
      parsed.data.username,
      parsed.data.email,
      parsed.data.password,
    );
    const json = await response.json().catch();
    if (!response.ok) {
      return { error: json.message || response.statusText };
    }
  } catch (err) {
    return { error: 'Something went wrong' };
  }
  redirect('/auth/login');
}
