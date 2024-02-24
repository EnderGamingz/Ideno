import API from '@/lib/api';
import { HStack } from '@/styling/jsx';
import Button from '@/recipes/button';
import Icon from '@/app/_components/icon';
import { DataUpdateForm } from '@/app/_components/Dialog/dataUpdateForm';
import updateUserAdminAction from '@/app/auth/admin/users/[id]/updateUserAction';
import { EditField } from '@/app/profile/_components/editField';
import LogoutAction from '@/app/_components/header/logoutAction';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import deleteUserAction from '@/app/auth/admin/users/[id]/deleteUserAction';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  const updateWithId = updateUserAdminAction.bind(null, selectedUser.id);
  return (
    <>
      <HStack>
        <h2>Showing details for:</h2>
        <h3>&quot;{selectedUser.username}&quot;</h3>
      </HStack>
      <HStack mb={2}>
        <ConfirmPopover
          label={'Are you sure you want to delete this user?'}
          buttonEl={'Delete'}
          buttonType={'button'}
          popoverSide={'right'}
          confirm={{
            action: deleteUserAction,
            actionPayload: selectedUser.id,
            button: (
              <>
                <Icon.Delete /> Delete
              </>
            ),
          }}
        />
      </HStack>
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
