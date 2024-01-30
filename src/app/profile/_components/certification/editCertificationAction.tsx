'use server';

import { z } from 'zod';
import API from '@/lib/api';
import { customDate } from '@/utils/parsing';

const schema = z.object({
  name: z.string().min(1),
  organization: z.string().min(1),
  issue_date: customDate.optional(),
  expiration_date: customDate.optional(),
  credential_id: z.string().optional(),
  credential_url: z.string().optional(),
});

export default async function editCertificationAction(
  formData: FormData,
  id: number,
) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return API.auth.profile.certification
    .updateById(id, parsed.data)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
