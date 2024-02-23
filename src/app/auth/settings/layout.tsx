import auth from 'auth';
import { redirect } from 'next/navigation';
import Icon from '@/app/_components/icon';
import { Divider, Grid, GridItem, Stack, styled } from '@/styling/jsx';
import { ReactNode } from 'react';
import Link from 'next/link';
import { hstack } from '@/styling/patterns';

const settingsPages = [
  {
    title: 'Profile',
    icon: Icon.Profile,
    path: 'profile',
  },
  {
    title: 'Account',
    icon: Icon.Settings,
    path: 'account',
  },
];

export default async function Layout({ children }: { children: ReactNode }) {
  const user = await auth();
  if (!user) redirect('/auth/login');

  return (
    <Grid columns={{ base: 1, md: 5 }}>
      <GridItem colSpan={{ base: 1, md: 2 }} css={{}}>
        <Stack p={3} gap={3}>
          <styled.h1 fontSize={'2rem'}>Settings</styled.h1>
          <Divider color={'gray.300'} />
          {settingsPages.map(page => (
            <Link
              className={hstack({
                fontSize: '1.2rem',
                p: 1.5,
                rounded: 'lg',
                outline: '1px solid',
                oct: 'black/80',
              })}
              key={page.path}
              href={'/auth/settings/' + page.path}>
              <page.icon size={25} />
              {page.title}
            </Link>
          ))}
        </Stack>
      </GridItem>
      <GridItem colSpan={{ base: 1, md: 3 }}>{children}</GridItem>
    </Grid>
  );
}
