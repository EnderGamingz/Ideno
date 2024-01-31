'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import { AuthExperienceModel } from '@/types/experience';
import editExperienceAction from '@/app/profile/_components/experience/editEducationAction';

export default function EditExperienceDialog({
  data,
}: {
  data: AuthExperienceModel;
}) {
  if (!data.id) return null;
  const editActionWithId = editExperienceAction.bind(null, data.id);
  return (
    <EditDataDialog
      title={'Update Experience'}
      description={'Enter your new experience details'}
      button={<Icon.Edit size={18} />}
      action={editActionWithId}>
      <EditField
        label={'Title'}
        fieldId={'title'}
        required
        value={data.title}
      />
      <EditField
        label={'Company'}
        fieldId={'company'}
        required
        value={data.company}
      />
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
      TODO: EXP Type
    </EditDataDialog>
  );
}
