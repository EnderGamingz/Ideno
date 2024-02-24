'use server';
import { z } from 'zod';
import API from '@/lib/api';

const schema = z.object({
  username: z.string(),
  email: z.string().email(),
  role: z.string().min(1),
});

export default async function updateUserAdminAction(
  id: number,
  formData: FormData,
) {
  const objectFromFormData = Object.fromEntries(formData.entries());
  const parsed = schema.safeParse(objectFromFormData);
  if (!parsed.success) {
    return { errors: parsed.error.errors };
  }

  return await API.auth.admin.user.updateById(id, parsed.data).then(res => {
    if (res?.error) {
      return {
        error: 'Internal Server Error',
      };
    }
    return { success: true };
  });
}
