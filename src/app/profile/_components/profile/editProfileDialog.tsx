'use client';

import Icon from '@/app/_components/icon';
import { PublicProfileModel } from '@/types/profile';
import profileUpdateAction from '@/app/profile/_components/profile/profileUpdateAction';
import { EditField } from '@/app/profile/_components/editField';
import { SelectFromOptions } from '@/app/_components/selectFromOptions';
import EditDataDialog from '@/app/_components/Dialog/editDataDialog';
import { Grid } from '@/styling/jsx';

export default function EditProfileDialog({
  profile,
}: {
  profile: PublicProfileModel;
}) {
  return (
    <EditDataDialog
      title={'Edit Profile Information'}
      description={'Here you can update your profile details'}
      button={<Icon.Edit />}
      action={profileUpdateAction}>
      <EditField
        label={'First Name'}
        fieldId={'first_name'}
        value={profile.first_name}
      />
      <EditField
        label={'Last Name'}
        fieldId={'last_name'}
        value={profile.last_name}
      />
      <EditField
        label={'Headline'}
        fieldId={'headline'}
        value={profile.headline}
      />
      <Grid columns={{ base: 1, md: 2 }}>
        <EditField label={'City'} fieldId={'city'} value={profile.city} />
        <EditField
          label={'Country'}
          fieldId={'country'}
          value={profile.country}
        />
      </Grid>
      <SelectFromOptions
        initial={profile.pronouns}
        valueOptions={[
          ['he', 'he/him'],
          ['she', 'she/her'],
          ['they', 'they/them'],
        ]}
        label={'Pronouns'}
        fieldId={'pronouns'}
        allowCustom
        allowNone
      />
      <EditField
        type={'textarea'}
        label={'Bio'}
        fieldId={'bio'}
        value={profile.bio}
      />
    </EditDataDialog>
  );
}
