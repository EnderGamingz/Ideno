'use server';
import API from '@/lib/api';

export default async function deleteContactInformationAction(
  id: string,
): Promise<{ error: boolean } | { success: boolean }> {
  return await API.auth.admin.contactInformation.deleteById(id);
}
