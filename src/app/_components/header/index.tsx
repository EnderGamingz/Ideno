import { Grid, HStack, styled } from '@/styling/jsx';
import Image from 'next/image';
import Link from 'next/link';
import { button } from '@/recipes/button';
import auth from 'auth';
import LogoutButton from '@/app/_components/header/logoutButton';
import Icon from '@/app/_components/icon';

export default async function Header() {
  const user = await auth();
  return (
    <styled.header
      css={{
        display: 'grid',
        alignItems: 'center',
        p: 3,
        px: 5,
        gridTemplateColumns: 'repeat(3, 1fr)',
        gap: '1rem',
        zIndex: 1000,
        pos: 'relative',
        bg: 'background',
        boxShadow: '0 0 5px -1px rgba(0,0,0,0.1)',
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
      <HStack gap={3} justifyContent={'flex-end'}>
        {user ? (
          <>
            <Link className={button()} href={`/profile/${user.username}`}>
              Profile
            </Link>
            <Link
              className={button({ contentType: 'icon' })}
              href={`/auth/settings`}>
              <Icon.Settings />
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
