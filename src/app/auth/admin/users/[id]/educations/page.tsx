import API from '@/lib/api';
import { AdminTable } from '@/app/auth/admin/_components/adminTable';
import Icon from '@/app/_components/icon';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import { AuthEducationModel } from '@/types/education';
import deleteContactInformationAction from '@/app/auth/admin/users/[id]/educations/deleteContactInformationAction';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  const educations = await API.profile.getEducationByUsername(
    selectedUser.username,
  );

  return (
    <>
      <h2>Education List for &quot;{selectedUser.username}&quot;</h2>
      <AdminTable
        header={[
          'ID',
          'Degree',
          'Field',
          'School',
          'Start Date',
          'End Date',
          'Action',
        ]}
        dataList={educations ?? []}
        callbackFn={(education: AuthEducationModel) => (
          <tr key={education.id}>
            <td>{education.id}</td>
            <td>{education.degree}</td>
            <td>{education.field}</td>
            <td>{education.school}</td>
            <td>{education.start_date}</td>
            <td>{education.end_date}</td>
            <td>
              <ConfirmPopover
                label={'Are you sure you want to delete this education?'}
                buttonEl={'Delete'}
                buttonType={'button'}
                confirm={{
                  action: deleteContactInformationAction,
                  actionPayload: education.id,
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
