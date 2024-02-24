'use server';
import API from '@/lib/api';

export default async function accountDeleteAction() {
  return await API.auth.account.delete().then(res => {
    if (res.error) {
      return {
        error: 'Error',
      };
    }
  });
}
