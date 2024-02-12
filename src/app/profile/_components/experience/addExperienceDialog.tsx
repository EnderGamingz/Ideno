'use client';

import Icon from '@/app/_components/icon';
import { EditField } from '@/app/profile/_components/editField';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import addExperienceAction from '@/app/profile/_components/experience/addExperienceAction';
import { SelectFromOptions } from '@/app/_components/selectFromOptions';

export const experienceTypes = [
  ['Full Time', 'Full Time'],
  ['Part Time', 'Part Time'],
  ['Self Employed', 'Self Employed'],
  ['Freelance', 'Freelance'],
  ['Contract', 'Contract'],
  ['Internship', 'Internship'],
  ['Volunteering', 'Volunteering'],
  ['Seasonal', 'Seasonal'],
  ['Apprenticeship', 'Apprenticeship'],
  ['Other', 'Other'],
];

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
      <SelectFromOptions
        valueOptions={experienceTypes}
        label={'Experience Type'}
        fieldId={'exp_type'}
        allowNone
      />
    </EditDataDialog>
  );
}
