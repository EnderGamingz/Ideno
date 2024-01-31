'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import editCertificationAction from '@/app/profile/_components/certification/editCertificationAction';
import { AuthCertificationModel } from '@/types/certification';

export default function EditCertificationDialog({
  data,
}: {
  data: AuthCertificationModel;
}) {
  if (!data.id) return null;
  const editActionWithId = editCertificationAction.bind(null, data.id);
  return (
    <EditDataDialog
      title={'Update Certification'}
      description={'Enter your new certification details'}
      button={<Icon.Edit size={18} />}
      action={editActionWithId}>
      <EditField label={'Name'} fieldId={'name'} value={data.name} required />
      <EditField
        label={'Organization'}
        fieldId={'organization'}
        value={data.organization}
        required
      />
      <EditField
        label={'Issue Date'}
        fieldId={'issue_date'}
        value={data.issue_date}
        type={'date'}
      />
      <EditField
        label={'Expiry Date'}
        fieldId={'expiration_date'}
        value={data.expiration_date}
        type={'date'}
      />
      <EditField
        label={'Credential ID'}
        fieldId={'credential_id'}
        value={data.credential_id}
      />
      <EditField
        label={'Credential Url'}
        fieldId={'credential_url'}
        value={data.credential_url}
      />
    </EditDataDialog>
  );
}
