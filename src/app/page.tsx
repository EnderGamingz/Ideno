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
        h: 'min(100%, 600px)',
        maxW: '1600px',
        w: 'full',
        mx: 'auto',
        display: 'grid',
        gridTemplateColumns: { base: '1fr', lg: 'repeat(2, 1fr)' },
        color: 'text.primary',
        px: { base: '2rem', sm: '3rem', lg: '5rem' },
        bg: 'violet.50',
        boxShadow: '0 0 0 100vmax token(colors.violet.50)',
        clipPath: 'inset(0 -100vmax)',
      }}>
      <Stack gap={3} maxW={'650px'} py={'4rem'} overflow={'hidden'}>
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
      <Box
        css={{
          hideBelow: 'lg',
          display: 'grid',
          isolation: 'isolate',
          '& > *': {
            gridColumn: '1/-1',
            gridRow: '1/-1',
          },
        }}>
        <styled.svg
          alignSelf={'center'}
          zIndex={'-1'}
          transform={'translate(50px, -80px)'}
          viewBox={'0 0 200 200'}
          css={{
            w: { base: '500px', xl: '670px' },
            aspectRatio: '1/1',
            '& > path': {
              fill: 'violet.200',
              filter: 'drop-shadow(5px 5px rgba(0,0,0,0.7))',
            },
          }}
          xmlns={'http://www.w3.org/2000/svg'}>
          <path
            stroke={'black'}
            strokeWidth={'1.5'}
            strokeLinecap={'round'}
            strokeLinejoin={'round'}
            d='M41.5,-64.1C55.7,-63.7,70.4,-56.4,77.7,-44.6C85,-32.8,84.9,-16.4,84.5,-0.2C84.2,16,83.7,32,77.1,45C70.4,58,57.7,67.9,43.8,73.9C29.9,80,15,82.1,1.4,79.7C-12.1,77.2,-24.3,70.1,-34.9,62.2C-45.5,54.3,-54.6,45.5,-62.1,35C-69.5,24.5,-75.3,12.3,-76,-0.4C-76.7,-13.1,-72.3,-26.2,-65.2,-37.1C-58,-48.1,-48,-57,-36.7,-59C-25.4,-61,-12.7,-56.1,0.5,-57C13.7,-57.9,27.3,-64.5,41.5,-64.1Z'
            transform='translate(100 100)'
          />
        </styled.svg>
        <Image
          className={css({
            p: 5,
            alignSelf: 'center',
            justifySelf: 'center',
            transform: {
              base: 'translate(45px, -80px)',
              xl: 'translate(90px, -80px)}',
            },
            width: {
              lgDown: '450px',
              xl: '600px',
            },
          })}
          src={'/Information.svg'}
          alt={'share online graphic'}
          width={500}
          height={500}
        />
      </Box>
    </Box>
  );
}
