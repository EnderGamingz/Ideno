'use server';

import { z } from 'zod';
import API from '@/lib/api';

const schema = z.object({
  first_name: z.string().optional(),
  last_name: z.string().optional(),
  headline: z.string().optional(),
  city: z.string().optional(),
  country: z.string().optional(),
  bio: z.string().optional(),
  pronouns: z.string().optional(),
});

export default async function profileUpdateAction(formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return API.auth.profile
    .update(parsed.data)
    .then(() => {
      return { success: true };
    })
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
