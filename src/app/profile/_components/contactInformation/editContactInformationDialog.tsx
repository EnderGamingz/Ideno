'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import { SelectFromOptions } from '@/app/_components/selectFromOptions';
import { useState } from 'react';
import { contactTypes } from '@/app/profile/_components/contactInformation/contactInformationItem';
import { AuthContactInformationModel } from '@/types/contactInformation';
import editContactInformationAction from '@/app/profile/_components/contactInformation/editContactInformationAction';

export default function EditContactInformationDialog({
  data,
}: {
  data: AuthContactInformationModel;
}) {
  const [typeValue, setTypeValue] = useState<any>(undefined);
  if (!data.id) return null;
  const editActionWithId = editContactInformationAction.bind(null, data.id);

  return (
    <EditDataDialog
      title={'Update Contact Information'}
      description={'Enter your new contact details'}
      button={<Icon.Edit />}
      action={editActionWithId}>
      {data.type_field}
      <SelectFromOptions
        initial={data.type_field}
        valueOptions={contactTypes}
        label={'Contact Type'}
        fieldId={'contact_type'}
        onCurrentValueChange={setTypeValue}
        valueChangeIndex={2}
      />
      <EditField
        value={data.value}
        type={typeValue?.type ?? 'text'}
        label={'Value'}
        fieldId={'value'}
        prefix={typeValue?.visiblePrefix}
        required
      />
    </EditDataDialog>
  );
}
