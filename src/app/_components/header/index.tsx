import { Grid, HStack, styled } from '@/styling/jsx';
import Image from 'next/image';
import Link from 'next/link';
import { button } from '@/recipes/button';
import auth from 'auth';
import LogoutButton from '@/app/_components/header/logoutButton';

export default async function Header() {
  const user = await auth();
  return (
    <styled.header
      css={{
        display: 'grid',
        alignItems: 'center',
        p: 2,
        px: 4,
        gridTemplateColumns: 'repeat(3, 1fr)',
        gap: '1rem',
        borderBottom: '1px solid',
        bct: 'black/85',
        zIndex: 1000,
        pos: 'relative',
        bg: 'background',
      }}>
      <styled.nav
        css={{
          display: 'flex',
          alignItems: 'center',
          gap: 1,

          '& a': {
            transition: 'all 0.2s',
            rounded: 'sm',
            px: 2,
            py: 0.5,
            _hover: {
              bgt: 'secondary/75',
            },
          },
        }}>
        <Link href={'/'}>Home</Link>
        <Link href={'/about'}>About</Link>
        <Link href={'/contact'}>Contact</Link>
      </styled.nav>
      <Grid placeItems={'center'}>
        <Link href={'/'}>
          <Image src={'/Logo.svg'} alt={'Ideno Logo'} width={40} height={40} />
        </Link>
      </Grid>
      <HStack gap={2} justifyContent={'flex-end'}>
        {user ? (
          <>
            <Link className={button()} href={`/profile/${user.username}`}>
              Profile
            </Link>
            <LogoutButton />
          </>
        ) : (
          <>
            <Link
              className={button({ variant: 'outline' })}
              href={'/auth/login'}>
              Login
            </Link>
            <Link className={button()} href={'/auth/register'}>
              Register
            </Link>
          </>
        )}
      </HStack>
    </styled.header>
  );
}
