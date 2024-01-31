'use server';

import API from '@/lib/api';

export default async function deleteCertificationAction(id: number) {
  return API.auth.profile.certification
    .deleteById(id)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
