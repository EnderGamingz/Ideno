import { Box, HStack, Stack, styled } from '@/styling/jsx';
import { button } from '@/recipes/button';
import Link from 'next/link';
import Icon from '@/app/_components/icon';
import Image from 'next/image';
import { css } from '@/styling/css';

// prettier-ignore
function HeroTitle() {
  return (
    <styled.h1
      css={{
        fontSize: '5xl',
        fontWeight: 'bold',
        '& span': {
          filter: 'drop-shadow(3px 3px rgba(0,0,0,0.7))',
        },
      }}>
      <styled.span color={'primary'}>Collect</styled.span>
      ,{' '}
      <styled.span color={'secondary'}>Create</styled.span>
      {' '} and {' '}
      <styled.span color={'tertiary'}>Share</styled.span>
      {' '}
      your working experiences
    </styled.h1>
  );
}

export default async function Home() {
  return (
    <Box
      css={{
        h: 'min(100vh, 600px)',
        maxW: '1600px',
        w: 'full',
        mx: 'auto',
        display: 'grid',
        gridTemplateColumns: { base: '1fr', lg: 'repeat(2, 1fr)' },
        color: 'text.primary',
        px: { base: '2rem', sm: '3rem', lg: '5rem' },
        py: '4rem',
        bg: 'violet.50',
        boxShadow: '0 0 0 100vmax token(colors.violet.50)',
        clipPath: 'inset(0 -100vmax)',
      }}>
      <Stack gap={3} maxW={'650px'}>
        <HeroTitle />
        <styled.p
          css={{
            fontSize: '3xl',
            fontWeight: 'light',
          }}>
          Empower your professional growth with Ideno
        </styled.p>
        <HStack mt={'1rem'}>
          <Link href={'/auth/register'} className={button({ size: 'large' })}>
            Get Started <Icon.Forward />
          </Link>
        </HStack>
      </Stack>
      <Image
        className={css({
          hideBelow: 'lg',
          justifySelf: 'flex-end',
        })}
        src={'/meet_online.svg'}
        alt={'share online graphic'}
        width={600}
        height={500}
      />
    </Box>
  );
}
