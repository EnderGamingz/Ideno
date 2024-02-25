'use server';
import API from '@/lib/api';

export default async function deleteEducationAction(
  id: string,
): Promise<{ error: boolean } | { success: boolean }> {
  return await API.auth.admin.education.deleteById(id);
}
