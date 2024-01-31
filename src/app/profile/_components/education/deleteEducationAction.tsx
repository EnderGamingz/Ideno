'use server';

import API from '@/lib/api';

export default async function deleteEducationAction(id: number) {
  return API.auth.profile.education
    .deleteById(id)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
