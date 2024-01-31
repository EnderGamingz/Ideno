import { ReactNode } from 'react';
import { Box } from '@/styling/jsx';

export function Card({
  children,
  sticky,
}: {
  children: ReactNode;
  sticky?: boolean;
}) {
  return (
    <Box
      bg={'white'}
      p={2}
      shadow={'md'}
      rounded={'lg'}
      pos={sticky ? { base: 'static', md: 'sticky' } : undefined}
      top={3}
      css={{
        '& > * > h2': {
          fontSize: '1.2rem',
          fontWeight: 480,
        },
      }}>
      {children}
    </Box>
  );
}
