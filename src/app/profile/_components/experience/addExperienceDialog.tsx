'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import addExperienceAction from '@/app/profile/_components/experience/addExperienceAction';

export default function AddExperienceDialog() {
  return (
    <EditDataDialog
      title={'Add Experience'}
      description={'Enter your experience details'}
      button={<Icon.Add />}
      action={addExperienceAction}>
      <EditField label={'Title'} fieldId={'title'} required />
      <EditField label={'Company'} fieldId={'company'} required />
      <EditField label={'Start Date'} fieldId={'start_date'} type={'date'} />
      <EditField label={'End Date'} fieldId={'end_date'} type={'date'} />
      TODO: EXP Type
    </EditDataDialog>
  );
}
