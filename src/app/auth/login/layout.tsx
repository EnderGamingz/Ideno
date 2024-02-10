import React, { ReactNode } from 'react';
import { Box, styled } from '@/styling/jsx';
import { BackgroundBlobs } from '@/app/_components/backgroundBlobs';

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <Box display={'grid'} placeItems={'center'}>
      <BackgroundBlobs />
      <Box
        css={{
          mt: 10,
          p: 4,
          bg: 'surface',
          shadow: 'md',
          maxW: 'lg',
          w: 'full',
        }}>
        <styled.h1 fontSize={'2xl'}>Login to Ideno</styled.h1>
        {children}
      </Box>
    </Box>
  );
}
