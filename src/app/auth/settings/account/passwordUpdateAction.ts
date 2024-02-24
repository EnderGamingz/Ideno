'use server';
import { z } from 'zod';
import API from '@/lib/api';
import { revalidateTag } from 'next/cache';

const schema = z.object({
  password: z.string().min(1),
  oldPassword: z.string().min(1),
  passwordConfirm: z.string().min(1),
});

export default async function passwordUpdateAction(formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  if (parsed.data.password !== parsed.data.passwordConfirm) {
    return { error: 'Passwords do not match' };
  }

  return await API.auth.password
    .update({
      new_password: parsed.data.password,
      old_password: parsed.data.oldPassword,
    })
    .then(res => {
      if (res.error) {
        return {
          error: res.message,
        };
      }
      revalidateTag('auth');
      return { success: true };
    });
}
