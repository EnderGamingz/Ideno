'use client';

import { useFormState } from 'react-dom';
import registerSubmit from '@/app/auth/register/_components/registerAction';
import { Box } from '@/styling/jsx';
import { BackgroundBlobs } from '@/app/_components/backgroundBlobs';
import React from 'react';
import { CredentialForm } from '@/app/auth/_components/credentialForm';
import { CredentialInput } from '@/app/auth/_components/credentialInput';

export function RegisterForm() {
  const [state, formAction] = useFormState(registerSubmit, {
    errors: undefined,
    success: undefined,
    error: undefined,
  } as any);

  const isError = (field: string) => {
    if (Array.isArray(state.errors)) {
      return (
        state.errors.filter((x: any) => (x.path[0] || '') === field).length > 0
      );
    }
  };

  return (
    <Box display={'grid'} placeItems={'center'}>
      <BackgroundBlobs />
      <CredentialForm
        title={'Register'}
        formSubmitHandler={formAction}
        state={state}>
        <CredentialInput
          text='Username'
          type='username'
          name='username'
          inputId='username'
          error={isError('username')}
        />
        <CredentialInput
          text='Email'
          type='email'
          name='email'
          inputId='email'
          error={isError('email')}
        />
        <CredentialInput
          text='Password'
          type='password'
          name='password'
          inputId='password'
          error={isError('password')}
        />
      </CredentialForm>
    </Box>
  );
}
