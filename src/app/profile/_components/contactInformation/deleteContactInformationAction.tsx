'use server';

import API from '@/lib/api';

export default async function deleteContactInformationAction(id: number) {
  return API.auth.profile.contactInformation
    .deleteById(id)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
