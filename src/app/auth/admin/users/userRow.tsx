'use client';
import { UserModel } from '@/types/user';
import { useRouter } from 'next/navigation';

export function UserRow({ user }: { user: UserModel }) {
  const router = useRouter();
  const handleClick = () => {
    router.push('/auth/admin/users/' + user.id);
  };
  return (
    <tr onClick={handleClick}>
      <td>{user.id}</td>
      <td>{user.username}</td>
      <td>{user.email}</td>
      <td>{user.role}</td>
      <td>{user.created_at}</td>
    </tr>
  );
}
