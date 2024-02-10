'use client';

import { useFormState } from 'react-dom';
import loginSubmit from '@/app/auth/login/email/_components/loginAction';
import React, { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { CredentialForm } from '@/app/auth/_components/credentialForm';
import { CredentialInput } from '@/app/auth/_components/credentialInput';

export function LoginForm() {
  const router = useRouter();
  const [state, formAction] = useFormState(loginSubmit, {
    errors: undefined,
    success: undefined,
    error: undefined,
  } as any);

  useEffect(() => {
    let timeout: ReturnType<typeof setTimeout>;
    if (state.success) {
      timeout = setTimeout(() => {
        router.push('/');
      }, 700);
    }
    return () => {
      if (timeout) {
        clearTimeout(timeout);
      }
    };
  }, [router, state.success]);

  const isError = (field: string) => {
    if (Array.isArray(state.errors)) {
      return (
        state.errors.filter((x: any) => (x.path[0] || '') === field).length > 0
      );
    }
  };

  return (
    <CredentialForm
      title={'Email Login'}
      formSubmitHandler={formAction}
      state={state}
      backTo={'/auth/login'}>
      <CredentialInput
        text='Username'
        type='username'
        name='username'
        inputId='username'
        error={isError('username')}
      />
      <CredentialInput
        text='Password'
        type='password'
        name='password'
        inputId='password'
        error={isError('password')}
      />
    </CredentialForm>
  );
}
