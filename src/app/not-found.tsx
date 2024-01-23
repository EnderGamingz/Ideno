import { BackgroundBlobs } from '@/app/_components/backgroundBlobs';
import { Stack, styled } from '@/styling/jsx';
import Link from 'next/link';
import { button } from '@/recipes/button';
import Icon from './_components/icon';

export default function NotFound() {
  return (
    <>
      <BackgroundBlobs />
      <Stack
        minH={'50vh'}
        maxW={'sm'}
        mx={'auto'}
        justifyContent={'center'}
        textAlign={'center'}>
        <styled.h1
          css={{
            bgGradient: 'to-br',
            gradientFrom: 'cyan.500',
            gradientTo: 'red.500',
            bgClip: 'text',
            color: 'transparent',
          }}
          fontSize={'7rem'}
          fontWeight={900}>
          404
        </styled.h1>
        <styled.p mt={-4} mb={5} fontSize={'1.2rem'} fontWeight={300}>
          Sorry, we couldn&apos;t find this page.
        </styled.p>
        <Link className={button({ variant: 'outline' })} href={'/'}>
          <Icon.Back />
          Go Home
        </Link>
      </Stack>
    </>
  );
}
