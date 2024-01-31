'use server';

import { z } from 'zod';
import API from '@/lib/api';
import { customDate } from '@/utils/parsing';

const schema = z.object({
  school: z.string().min(1),
  degree: z.string().optional(),
  field: z.string().optional(),
  start_date: customDate.optional(),
  end_date: customDate.optional(),
});

export default async function addEducationAction(formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return API.auth.profile.education
    .create(parsed.data)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
