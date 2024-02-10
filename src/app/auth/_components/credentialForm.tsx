import { ZodIssue } from 'zod';
import React, { ReactNode } from 'react';
import { Box, HStack, styled } from '@/styling/jsx';
import Alert from '@/recipes/alert';
import Icon from '@/app/_components/icon';

import { SubmitButton } from '@/app/auth/login/email/_components/submitButton';
import Link from 'next/link';
import { button } from '@/recipes/button';
import ConditionalWrapper from '@/app/_components/ConditionalWrapper';

export function CredentialForm({
  title,
  formSubmitHandler,
  state,
  children,
  backTo,
  noSubmit,
}: {
  title: string;
  formSubmitHandler?: (payload: FormData) => void;
  state?: { errors?: ZodIssue[]; error?: string; success?: boolean };
  children: ReactNode | ReactNode[];
  backTo?: string;
  noSubmit?: boolean;
}) {
  return (
    <ConditionalWrapper
      condition={!!formSubmitHandler}
      wrapper={c => <styled.form action={formSubmitHandler}>{c}</styled.form>}>
      <Box
        css={{
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
          '& button,a': {
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
        }}>
        <styled.h2 fontSize={'1xl'} fontWeight={'light'} mb={2}>
          {title}
        </styled.h2>
        <Alert
          status={
            state?.error ? 'error' : state?.success ? 'success' : 'hidden'
          }>
          {state?.success ? <Icon.Check size={20} /> : <Icon.Error size={20} />}
          {state?.error ? state?.error : state?.success ? 'Success' : ''}
        </Alert>
        {children}
        <HStack
          alignItems={'center'}
          justify={backTo ? 'space-between' : 'flex-end'}>
          {backTo && (
            <Link className={button({ variant: 'secondary' })} href={backTo}>
              Back
            </Link>
          )}
          {!noSubmit && <SubmitButton disabled={!!state?.success} />}
        </HStack>
      </Box>
    </ConditionalWrapper>
  );
}
