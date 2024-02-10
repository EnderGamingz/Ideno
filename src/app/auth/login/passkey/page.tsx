'use client';
import { CredentialForm } from '@/app/auth/_components/credentialForm';
import React from 'react';
import Button from '@/recipes/button';
import { Stack } from '@/styling/jsx';

export default function Page() {
  return (
    <CredentialForm title={'Passkey Login'} backTo={'/auth/login'} noSubmit>
      <Stack>
        <Button>Register</Button>
        <Button>Authenticate</Button>
      </Stack>
    </CredentialForm>
  );
}
