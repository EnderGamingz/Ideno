'use client';

import { useFormState, useFormStatus } from 'react-dom';
import loginSubmit from '@/app/auth/login/_components/loginAction';
import { Box, styled } from '@/styling/jsx';
import Button from '@/recipes/button';
import Icon from '@/app/_components/icon';
import Alert from '@/recipes/alert';
import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { BackgroundBlobs } from '@/app/_components/backgroundBlobs';

export function LoginForm() {
  const router = useRouter();
  const [state, formAction] = useFormState(loginSubmit, {
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
  }, [state.success]);

  return (
    <Box
      css={{
        display: 'grid',
        placeItems: 'center',
      }}>
      <BackgroundBlobs />
      <styled.form
        css={{
          mt: 10,
          p: 4,
          bg: 'surface',
          shadow: 'md',
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
            _focus: {
              outline: 'none',
              borderColor: 'primary',
            },
            _invalid: {
              borderColor: 'secondary',
            },
          },
        }}
        action={formAction}>
        <styled.h1 fontSize={'2xl'} fontWeight={'light'} mb={2}>
          Login to your account
        </styled.h1>
        <Alert
          status={state.error ? 'error' : state.success ? 'success' : 'hidden'}>
          {state.success ? <Icon.Check size={20} /> : <Icon.Error size={20} />}
          {state.error ? state.error : state.success ? 'Success' : ''}
        </Alert>
        <styled.section mt={1}>
          <label htmlFor='username'>Email</label>
          <input
            autoComplete='username'
            autoFocus
            type='text'
            name='username'
            id='username'
            required
          />
          {isError('username') && <span>This field is invalid</span>}
        </styled.section>
        <section>
          <label htmlFor='password'>Password</label>
          <input
            autoComplete='current-password'
            type='password'
            name='password'
            id='password'
            required
          />
          {isError('password') && <span>This field is invalid</span>}
        </section>
        <SubmitButton disabled={state.success} />
      </styled.form>
    </Box>
  );
}

function SubmitButton({ disabled }: { disabled: boolean }) {
  const { pending } = useFormStatus();
  return (
    <Button disabled={disabled} pending={pending} type='submit'>
      Submit
      <Icon.Login />
    </Button>
  );
}
