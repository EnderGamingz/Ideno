import { styled } from '@/styling/jsx';
import { DataUpdateForm } from '@/app/_components/Dialog/dataUpdateForm';
import { EditField } from '@/app/profile/_components/editField';
import auth from 'auth';
import { redirect } from 'next/navigation';
import accountUpdateAction from '@/app/auth/settings/account/accountUpdateAction';

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
      <DataUpdateForm action={accountUpdateAction} cancel={false}>
        <EditField label={'Email'} fieldId={'email'} value={account.email} />
      </DataUpdateForm>
    </>
  );
}
