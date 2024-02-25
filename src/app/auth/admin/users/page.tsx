import API from '@/lib/api';
import { TableLinkRow } from '@/app/auth/admin/_components/tableLinkRow';
import { UserModel } from '@/types/user';
import { AdminTable } from '@/app/auth/admin/_components/adminTable';

export default async function Page() {
  const users = await API.auth.admin.user.getAll();
  return (
    <>
      <h2>User List</h2>
      <AdminTable
        header={['ID', 'Username', 'Email', 'role', 'Created At']}
        dataList={users}
        callbackFn={(user: UserModel) => (
          <TableLinkRow key={user.id} href={'/auth/admin/users/' + user.id}>
            <td>{user.id}</td>
            <td>{user.username}</td>
            <td>{user.email}</td>
            <td>{user.role}</td>
            <td>{user.created_at}</td>
          </TableLinkRow>
        )}
      />
    </>
  );
}
