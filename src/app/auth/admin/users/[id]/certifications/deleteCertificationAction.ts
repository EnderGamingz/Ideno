'use server';
import API from '@/lib/api';

export default async function deleteCertificationAction(
  id: string,
): Promise<{ error: boolean } | { success: boolean }> {
  return await API.auth.admin.certification.deleteById(id);
}
