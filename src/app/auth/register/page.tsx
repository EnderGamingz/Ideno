import { RegisterForm } from '@/app/auth/register/_components/registerForm';
import { Box, styled } from '@/styling/jsx';
import React from 'react';

export default function Page() {
  return (
    <Box display={'grid'} placeItems={'center'}>
      <Box
        css={{
          mt: 10,
          p: 4,
          bg: 'surface',
          shadow: 'md',
          maxW: 'lg',
          w: 'full',
        }}>
        <styled.h1 fontSize={'2xl'}>Join Ideno</styled.h1>
        <RegisterForm />
      </Box>
    </Box>
  );
}
