import { ReactNode } from 'react';
import { admin } from 'auth';
import { ForbiddenScreen } from '@/app/auth/admin/_components/forbiddenScreen';
import { Box, Container, HStack, styled } from '@/styling/jsx';
import Icon from '@/app/_components/icon';
import Link from 'next/link';
import { button } from '@/recipes/button';

const links = [
  {
    name: 'Users',
    href: '/auth/admin/users',
    icon: Icon.Profile,
  },
];

export default async function Layout({ children }: { children: ReactNode }) {
  const user = await admin();
  if (!user) return <ForbiddenScreen />;

  return (
    <>
      <Box p={4} mb={4} borderBottom={'1px solid'} bct={'black/90'}>
        <styled.h1 fontSize={'3xl'} fontWeight={'semibold'}>
          Admin Area
        </styled.h1>
        <HStack mt={3} flexWrap={'wrap'}>
          {links.map(link => (
            <Link
              className={button({ variant: 'secondary' })}
              key={link.href}
              href={link.href}>
              {link.name}
            </Link>
          ))}
        </HStack>
      </Box>
      <Container
        w={'full'}
        maxW={'4xl'}
        css={{
          '& h2': {
            fontSize: '2xl',
            mb: 2,
          },
          '& h3': {
            fontSize: 'xl',
            mb: 2,
          },
        }}>
        {children}
      </Container>
    </>
  );
}
