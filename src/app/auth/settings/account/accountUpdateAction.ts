'use server';
import { z } from 'zod';
import API from '@/lib/api';
import { revalidateTag } from 'next/cache';

const schema = z.object({
  username: z.string().optional(),
  email: z.string().email().optional(),
});

export default async function accountUpdateAction(formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return await API.auth.account.update(parsed.data).then(res => {
    if (res.error) {
      return {
        error: res.message,
      };
    }
    revalidateTag('auth');
    return { success: true };
  });
}
