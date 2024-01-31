'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import addCertificationAction from '@/app/profile/_components/certification/addCertificationAction';

export default function AddCertificationDialog() {
  return (
    <EditDataDialog
      title={'Add Certification'}
      description={'Enter your certification details'}
      button={<Icon.Add />}
      action={addCertificationAction}>
      <EditField label={'Name'} fieldId={'name'} required />
      <EditField label={'Organization'} fieldId={'organization'} required />
      <EditField label={'Issue Date'} fieldId={'issue_date'} type={'date'} />
      <EditField
        label={'Expiry Date'}
        fieldId={'expiration_date'}
        type={'date'}
      />
      <EditField label={'Credential ID'} fieldId={'credential_id'} />
      <EditField label={'Credential Url'} fieldId={'credential_url'} />
    </EditDataDialog>
  );
}
