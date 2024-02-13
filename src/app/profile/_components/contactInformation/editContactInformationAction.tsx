'use server';

import { z } from 'zod';
import API from '@/lib/api';

const schema = z.object({
  contact_type: z.string().min(1),
  value: z.string().min(1),
});

export default async function editContactInformationAction(
  id: number,
  formData: FormData,
) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return API.auth.profile.contactInformation
    .updateById(id, parsed.data)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
