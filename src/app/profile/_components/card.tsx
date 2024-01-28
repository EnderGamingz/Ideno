import { ReactNode } from 'react';
import { Box } from '@/styling/jsx';

export function Card({ children }: { children: ReactNode }) {
  return (
    <Box
      bg={'white'}
      p={2}
      shadow={'md'}
      rounded={'lg'}
      pos={{ base: 'static', md: 'sticky' }}
      top={3}>
      {children}
    </Box>
  );
}