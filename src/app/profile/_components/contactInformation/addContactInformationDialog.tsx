'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import addContactInformationAction from '@/app/profile/_components/contactInformation/addContactInformationAction';
import { SelectFromOptions } from '@/app/_components/selectFromOptions';
import { useState } from 'react';
import { contactTypes } from '@/app/profile/_components/contactInformation/contactInformationItem';

export default function AddContactInformationDialog() {
  const [typeValue, setTypeValue] = useState<any>(undefined);
  return (
    <EditDataDialog
      title={'Add Contact Information'}
      description={'Enter your contact details'}
      button={<Icon.Add />}
      action={addContactInformationAction}>
      <SelectFromOptions
        valueOptions={contactTypes}
        label={'Contact Type'}
        fieldId={'contact_type'}
        onCurrentValueChange={setTypeValue}
        valueChangeIndex={2}
      />
      <EditField
        type={typeValue?.type ?? 'text'}
        label={'Value'}
        fieldId={'value'}
        prefix={typeValue?.visiblePrefix}
        required
      />
    </EditDataDialog>
  );
}
