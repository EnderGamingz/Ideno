import API from '@/lib/api';
import { AdminTable } from '@/app/auth/admin/_components/adminTable';
import Icon from '@/app/_components/icon';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import deleteContactInformationAction from '@/app/auth/admin/users/[id]/educations/deleteContactInformationAction';
import { AuthContactInformationModel } from '@/types/contactInformation';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  const contactInfoList = await API.profile.getContactInformationByUsername(
    selectedUser.username,
  );

  return (
    <>
      <h2>Contact Information List for &quot;{selectedUser.username}&quot;</h2>
      <AdminTable
        header={['ID', 'Type', 'Value', 'Action']}
        dataList={contactInfoList ?? []}
        callbackFn={(contactInfo: AuthContactInformationModel) => (
          <tr key={contactInfo.id}>
            <td>{contactInfo.id}</td>
            <td>{contactInfo.type_field}</td>
            <td>{contactInfo.value}</td>
            <td>
              <ConfirmPopover
                label={
                  'Are you sure you want to delete this contact information?'
                }
                buttonEl={'Delete'}
                buttonType={'button'}
                confirm={{
                  action: deleteContactInformationAction,
                  actionPayload: contactInfo.id,
                  refresh: true,
                  button: (
                    <>
                      <Icon.Delete /> Delete
                    </>
                  ),
                }}
              />
            </td>
          </tr>
        )}
      />
    </>
  );
}
