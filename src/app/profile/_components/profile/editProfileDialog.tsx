'use client';

import Icon from '@/app/_components/icon';
import { Stack } from '@/styling/jsx';
import { PublicProfileModel } from '@/types/profile';
import profileUpdateAction from '@/app/profile/_components/profile/profileUpdateAction';
import { EditField } from '@/app/profile/_components/editField';
import { SelectPronouns } from '@/app/profile/_components/profile/selectPronouns';
import EditDataDialog from '@/app/_components/editDataDialog';

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
      <Stack flexDirection={{ base: 'column', sm: 'row' }}>
        <EditField label={'City'} fieldId={'city'} value={profile.city} />
        <EditField
          label={'Country'}
          fieldId={'country'}
          value={profile.country}
        />
      </Stack>
      <SelectPronouns initial={profile.pronouns} />
      <EditField
        type={'textarea'}
        label={'Bio'}
        fieldId={'bio'}
        value={profile.bio}
      />
    </EditDataDialog>
  );
}
