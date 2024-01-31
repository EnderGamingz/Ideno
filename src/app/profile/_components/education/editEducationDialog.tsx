'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import { AuthEducationModel } from '@/types/education';
import editEducationAction from '@/app/profile/_components/education/editEducationAction';

export default function EditEducationDialog({
  data,
}: {
  data: AuthEducationModel;
}) {
  if (!data.id) return null;
  const editActionWithId = editEducationAction.bind(null, data.id);
  return (
    <EditDataDialog
      title={'Update Education'}
      description={'Enter your new education details'}
      button={<Icon.Edit size={18} />}
      action={editActionWithId}>
      <EditField
        label={'School'}
        fieldId={'school'}
        required
        value={data.school}
      />
      <EditField
        label={'Field of Study'}
        fieldId={'field'}
        value={data.field}
      />
      <EditField label={'Degree'} fieldId={'degree'} value={data.degree} />
      <EditField
        label={'Start Date'}
        fieldId={'start_date'}
        type={'date'}
        value={data.start_date}
      />
      <EditField
        label={'End Date'}
        fieldId={'end_date'}
        type={'date'}
        value={data.end_date}
      />
    </EditDataDialog>
  );
}
