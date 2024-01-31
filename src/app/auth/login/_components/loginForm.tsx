'use client';

import { useFormState, useFormStatus } from 'react-dom';
import loginSubmit from '@/app/auth/login/_components/loginAction';
import { Box, styled } from '@/styling/jsx';
import Button from '@/recipes/button';
import Icon from '@/app/_components/icon';
import Alert from '@/recipes/alert';
import React, { ReactNode, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { BackgroundBlobs } from '@/app/_components/backgroundBlobs';
import { ZodIssue } from 'zod';

export function CredentialForm({
  title,
  formSubmitHandler,
  state,
  children,
  inDialog,
}: {
  title: string;
  formSubmitHandler: (payload: FormData) => void;
  state: { errors?: ZodIssue[]; error?: string; success?: boolean };
  children: ReactNode | ReactNode[];
  inDialog?: boolean;
}) {
  return (
    <styled.form
      css={{
        mt: 10,
        p: 4,
        bg: !inDialog ? 'surface' : undefined,
        shadow: !inDialog ? 'md' : undefined,
        maxW: 'lg',
        w: 'full',
        rounded: 'md',
        '& section': {
          display: 'grid',
          mb: 2,
        },
        '& section label': {
          mb: 1,
          color: 'text',
        },
        '& section span': {
          color: 'error',
        },
        '& button': {
          mt: 3,
          float: 'right',
        },
        '& input': {
          border: '1px solid',
          borderColor: 'transparent',
          py: 1,
          px: 2,
          rounded: 'sm',
          transition: 'all 0.2s ease-in-out',
          bct: 'black/90',
          _focus: {
            outline: 'none',
            borderColor: 'primary!',
          },
          _invalid: {
            borderColor: 'secondary!',
          },
        },
      }}
      action={formSubmitHandler}>
      <styled.h1 fontSize={'2xl'} fontWeight={'light'} mb={2}>
        {title}
      </styled.h1>
      <Alert
        status={state?.error ? 'error' : state?.success ? 'success' : 'hidden'}>
        {state?.success ? <Icon.Check size={20} /> : <Icon.Error size={20} />}
        {state?.error ? state?.error : state?.success ? 'Success' : ''}
      </Alert>
      {children}
      <SubmitButton disabled={!!state?.success} />
    </styled.form>
  );
}

export function LoginForm({ inDialog }: { inDialog?: boolean }) {
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
    <Box display={'grid'} placeItems={'center'}>
      {!inDialog && <BackgroundBlobs />}
      <CredentialForm
        inDialog={inDialog}
        title={'Login to your account'}
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

export function CredentialInput({
  text,
  type,
  name,
  inputId,
  error,
}: {
  text: string;
  type?: string;
  name: string;
  inputId: string;
  error?: boolean;
}) {
  return (
    <styled.section mt={1}>
      <label htmlFor={inputId}>{text}</label>
      <input type={type ?? 'text'} name={name} id={inputId} required />
      {error && <span>This field is invalid</span>}
    </styled.section>
  );
}

export function SubmitButton({ disabled }: { disabled: boolean }) {
  const { pending } = useFormStatus();
  return (
    <Button disabled={disabled} pending={pending} type='submit'>
      Submit
      <Icon.Login />
    </Button>
  );
}
