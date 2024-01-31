'use client';

import { useFormState } from 'react-dom';
import registerSubmit from '@/app/auth/register/_components/registerAction';
import { Box } from '@/styling/jsx';
import { BackgroundBlobs } from '@/app/_components/backgroundBlobs';
import React from 'react';
import {
  CredentialForm,
  CredentialInput,
} from '@/app/auth/login/_components/loginForm';

export function RegisterForm({ inDialog }: { inDialog?: boolean }) {
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
    <Box
      css={{
        display: 'grid',
        placeItems: 'center',
      }}>
      {!inDialog && <BackgroundBlobs />}
      <CredentialForm
        inDialog={inDialog}
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
