'use server';
import { z } from 'zod';
import API from '@/lib/api';
import { revalidatePath } from 'next/cache';
import { redirect } from 'next/navigation';

export default async function deleteUserAction(id: number) {
  return await API.auth.admin.user.deleteById(id).then(res => {
    if (res?.error) {
      return {
        error: 'Internal Server Error',
      };
    }
    redirect('/auth/admin/users');
    return { success: true };
  });
}
