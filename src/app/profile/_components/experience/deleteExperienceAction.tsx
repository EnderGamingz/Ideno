'use server';

import API from '@/lib/api';

export default async function deleteExperienceAction(id: number) {
  return API.auth.profile.experience
    .deleteById(id)
    .then(() => ({ success: true }))
    .catch(() => {
      return { error: 'Something went wrong' };
    });
}
