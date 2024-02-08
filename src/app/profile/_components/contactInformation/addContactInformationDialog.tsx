'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import addContactInformationAction from '@/app/profile/_components/contactInformation/addContactInformationAction';
import { SelectFromOptions } from '@/app/_components/selectFromOptions';

export default function AddContactInformationDialog() {
  return (
    <EditDataDialog
      title={'Add Contact Information'}
      description={'Enter your contact details'}
      button={<Icon.Add />}
      action={addContactInformationAction}>
      <SelectFromOptions
        valueOptions={[
          ['email', 'Email'],
          ['phone', 'Phone'],
          ['website', 'Website'],
          ['linkedin', 'LinkedIn'],
          ['github', 'GitHub'],
          ['twitter', 'Twitter'],
          ['facebook', 'Facebook'],
          ['instagram', 'Instagram'],
        ]}
        label={'Contact Type'}
        fieldId={'contact_type'}
      />

      <EditField label={'Value'} fieldId={'value'} required />
    </EditDataDialog>
  );
}
