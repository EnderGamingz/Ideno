import API from '@/lib/api';
import { AdminTable } from '@/app/auth/admin/_components/adminTable';
import { AuthExperienceModel } from '@/types/experience';
import deleteExperienceAction from '@/app/auth/admin/users/[id]/experiences/deleteExperienceAction';
import Icon from '@/app/_components/icon';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  const experiences = await API.profile.getExperienceByUsername(
    selectedUser.username,
  );

  return (
    <>
      <h2>Experience List for &quot;{selectedUser.username}&quot;</h2>
      <AdminTable
        header={[
          'ID',
          'Title',
          'Company',
          'Description',
          'Start Date',
          'End Date',
          'Experience Type',
          'Action',
        ]}
        dataList={experiences ?? []}
        callbackFn={(experience: AuthExperienceModel) => (
          <tr key={experience.id}>
            <td>{experience.id}</td>
            <td>{experience.title}</td>
            <td>{experience.company}</td>
            <td>{experience.description}</td>
            <td>{experience.start_date}</td>
            <td>{experience.end_date}</td>
            <td>{experience.exp_type}</td>
            <td>
              <ConfirmPopover
                label={'Are you sure you want to delete this experience?'}
                buttonEl={'Delete'}
                buttonType={'button'}
                confirm={{
                  action: deleteExperienceAction,
                  actionPayload: experience.id,
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
