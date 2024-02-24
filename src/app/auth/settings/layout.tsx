import auth from 'auth';
import { redirect } from 'next/navigation';
import Icon from '@/app/_components/icon';
import {
  Box,
  Container,
  Divider,
  Grid,
  GridItem,
  Stack,
  styled,
} from '@/styling/jsx';
import { ReactNode } from 'react';
import Link from 'next/link';
import { hstack } from '@/styling/patterns';
import LogoutAction from '@/app/_components/header/logoutAction';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';

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
    <Grid columns={{ base: 1, md: 5 }} flexGrow={1}>
      <GridItem colSpan={{ base: 1, md: 2, xl: 1 }} css={{}}>
        <Stack p={3} gap={3} shadow={'md'} h={'full'}>
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
                transition: 'all 0.2s',
                _hover: {
                  bgct: 'primary/90',
                },
              })}
              key={page.path}
              href={'/auth/settings/' + page.path}>
              <page.icon size={25} />
              {page.title}
            </Link>
          ))}
          <Stack mt={'auto'} gap={0.5}>
            <styled.h3 fontWeight={'light'}>Logged in as</styled.h3>
            <styled.span fontWeight={'bold'}>{user.username}</styled.span>
            <Box mt={4}>
              <ConfirmPopover
                label={'Are you sure you want to logout?'}
                buttonEl={'Logout'}
                buttonType={'button'}
                popoverPosition={'bottom'}
                confirm={{
                  action: LogoutAction,
                  button: (
                    <>
                      <Icon.Logout /> Logout
                    </>
                  ),
                }}
              />
            </Box>
          </Stack>
        </Stack>
      </GridItem>
      <GridItem colSpan={{ base: 1, md: 3, xl: 4 }}>
        <Container mt={4} maxW={'3xl'}>
          {children}
        </Container>
      </GridItem>
    </Grid>
  );
}
