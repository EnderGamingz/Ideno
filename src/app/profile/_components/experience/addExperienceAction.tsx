'use server';

import { z } from 'zod';
import API from '@/lib/api';
import { customDate } from '@/utils/parsing';

const schema = z.object({
  company: z.string().min(1),
  title: z.string().min(1),
  start_date: customDate.optional(),
  end_date: customDate.optional(),
  exp_type: z.string().optional(),
  description: z.string().optional(),
});

export default async function addExperienceAction(formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return API.auth.profile.experience
    .create(parsed.data)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
