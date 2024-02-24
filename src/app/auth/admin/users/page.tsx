import API from '@/lib/api';
import { styled } from '@/styling/jsx';
import { UserRow } from '@/app/auth/admin/users/userRow';

export default async function Page() {
  const users = await API.auth.admin.user.getAll();
  return (
    <>
      <h2>UserList</h2>
      <styled.table
        w={'full'}
        css={{
          '& th,td': {
            textAlign: 'left',
            p: 1,
            outline: 'none',
          },
          '& th:not(:last-child),td:not(:last-child)': {
            borderRight: '1px solid lightgray',
          },
          '& thead': {
            _first: {
              borderBottom: '1px solid lightgray',
            },
          },
          '& tbody tr': {
            _odd: {
              bgct: 'primary/90',
            },
            cursor: 'pointer',
            _hover: {
              bgct: 'secondary/90',
            },
          },
        }}>
        <styled.thead>
          <tr>
            <th>ID</th>
            <th>Username</th>
            <th>Email</th>
            <th>role</th>
            <th>Created At</th>
          </tr>
        </styled.thead>
        <tbody>
          {users.map(user => (
            <UserRow user={user} key={user.id} />
          ))}
        </tbody>
      </styled.table>
    </>
  );
}
