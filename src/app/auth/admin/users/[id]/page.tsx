import API from '@/lib/api';
import { DataUpdateForm } from '@/app/_components/Dialog/dataUpdateForm';
import updateUserAdminAction from '@/app/auth/admin/users/[id]/updateUserAction';
import { EditField } from '@/app/profile/_components/editField';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  const updateWithId = updateUserAdminAction.bind(null, selectedUser.id);
  return (
    <>
      <DataUpdateForm action={updateWithId} cancel={false}>
        <EditField
          label={'Username'}
          fieldId={'username'}
          value={selectedUser.username}
        />
        <EditField
          label={'Email'}
          fieldId={'email'}
          value={selectedUser.email}
        />
        <EditField label={'Role'} fieldId={'role'} value={selectedUser.role} />
      </DataUpdateForm>
    </>
  );
}
