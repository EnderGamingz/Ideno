'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import addEducationAction from '@/app/profile/_components/education/addEducationAction';

export default function AddEducationDialog() {
  return (
    <EditDataDialog
      title={'Add Education'}
      description={'Enter your education details'}
      button={<Icon.Add />}
      action={addEducationAction}>
      <EditField label={'School'} fieldId={'school'} required />
      <EditField label={'Field of Study'} fieldId={'field'} />
      <EditField label={'Degree'} fieldId={'degree'} />
      <EditField label={'Start Date'} fieldId={'start_date'} type={'date'} />
      <EditField label={'End Date'} fieldId={'end_date'} type={'date'} />
    </EditDataDialog>
  );
}
