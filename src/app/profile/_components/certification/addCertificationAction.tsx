'use server';

import { z } from 'zod';
import API from '@/lib/api';

const customDate = z.string().refine(
  value => {
    if (value.length === 0) return true;
    return !isNaN(Date.parse(value));
  },
  {
    message: 'Invalid date format',
  },
);

const schema = z.object({
  name: z.string().min(1),
  organization: z.string().min(1),
  issue_date: customDate.optional(),
  expiration_date: customDate.optional(),
  credential_id: z.string().optional(),
  credential_url: z.string().optional(),
});

export default async function addCertificationAction(formData: FormData) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  console.log(objectFromFormData);
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return API.auth.profile.certification
    .create(parsed.data)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
