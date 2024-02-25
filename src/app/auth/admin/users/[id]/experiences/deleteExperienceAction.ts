'use server';
import API from '@/lib/api';

export default async function deleteExperienceAction(
  id: string,
): Promise<{ error: boolean } | { success: boolean }> {
  return await API.auth.admin.experience.deleteById(id);
}
