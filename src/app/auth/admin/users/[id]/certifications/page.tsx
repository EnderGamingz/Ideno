import API from '@/lib/api';
import { AdminTable } from '@/app/auth/admin/_components/adminTable';
import Icon from '@/app/_components/icon';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import { AuthCertificationModel } from '@/types/certification';
import deleteCertificationAction from '@/app/auth/admin/users/[id]/certifications/deleteCertificationAction';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  const certifications = await API.profile.getCertificationsByUsername(
    selectedUser.username,
  );

  return (
    <>
      <h2>Certification List for &quot;{selectedUser.username}&quot;</h2>
      <AdminTable
        header={[
          'ID',
          'Name',
          'Org',
          'Issue Date',
          'Exp Date',
          'Cred ID',
          'Cred URL',
          'Action',
        ]}
        dataList={certifications ?? []}
        callbackFn={(certification: AuthCertificationModel) => (
          <tr key={certification.id}>
            <td>{certification.id}</td>
            <td>{certification.name}</td>
            <td>{certification.organization}</td>
            <td>{certification.issue_date}</td>
            <td>{certification.expiration_date}</td>
            <td>{certification.credential_id}</td>
            <td>{certification.credential_url}</td>
            <td>
              <ConfirmPopover
                label={'Are you sure you want to delete this certification?'}
                buttonEl={'Delete'}
                buttonType={'button'}
                confirm={{
                  action: deleteCertificationAction,
                  actionPayload: certification.id,
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
