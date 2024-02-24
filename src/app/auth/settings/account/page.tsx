import { Divider, Grid, styled } from '@/styling/jsx';
import { DataUpdateForm } from '@/app/_components/Dialog/dataUpdateForm';
import { EditField } from '@/app/profile/_components/editField';
import auth from 'auth';
import { redirect } from 'next/navigation';
import accountUpdateAction from '@/app/auth/settings/account/accountUpdateAction';
import passwordUpdateAction from '@/app/auth/settings/account/passwordUpdateAction';
import { AccountDelete } from '@/app/auth/settings/account/accountDelete';

export default async function Page() {
  const account = await auth();
  if (!account) redirect('/auth/login');
  return (
    <>
      <styled.h2 fontSize={'3xl'}>Account Settings</styled.h2>
      <DataUpdateForm action={accountUpdateAction} cancel={false}>
        <EditField
          label={'Username'}
          fieldId={'username'}
          value={account.username}
        />
      </DataUpdateForm>
      <Divider my={5} />
      <DataUpdateForm action={accountUpdateAction} cancel={false}>
        <EditField label={'Email'} fieldId={'email'} value={account.email} />
      </DataUpdateForm>
      <Divider my={5} />
      <DataUpdateForm action={passwordUpdateAction} cancel={false}>
        <EditField
          label={'Old Password'}
          fieldId={'oldPassword'}
          type={'password'}
        />
        <Grid columns={{ base: 1, sm: 2 }}>
          <EditField
            label={'New Password'}
            fieldId={'password'}
            type={'password'}
          />
          <EditField
            label={'Confirm Password'}
            fieldId={'passwordConfirm'}
            type={'password'}
          />
        </Grid>
      </DataUpdateForm>
      <Divider my={5} />
      <AccountDelete />
    </>
  );
}
