import { HStack, Stack } from '@/styling/jsx';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import deleteUserAction from '@/app/auth/admin/users/[id]/deleteUserAction';
import Icon from '@/app/_components/icon';
import API from '@/lib/api';
import { ReactNode } from 'react';
import Link from 'next/link';
import { button } from '@/recipes/button';

const links = (id: string) => [
  {
    name: 'Account',
    href: `/auth/admin/users/${id}`,
  },
  {
    name: 'Certifications',
    href: `/auth/admin/users/${id}/certifications`,
  },
  {
    name: 'Educations',
    href: `/auth/admin/users/${id}/educations`,
  },
  {
    name: 'Experiences',
    href: `/auth/admin/users/${id}/experiences`,
  },
  {
    name: 'Contact Information',
    href: `/auth/admin/users/${id}/contact-information`,
  },
];

export default async function Layout({
  params,
  children,
}: {
  params: { id: string };
  children: ReactNode;
}) {
  const selectedUser = await API.auth.admin.user.getById(params.id);

  const userLinks = links(selectedUser.id.toString());

  return (
    <Stack>
      <HStack>
        <h2>Showing details for:</h2>
        <h3>&quot;{selectedUser.username}&quot;</h3>
      </HStack>
      <HStack mb={2}>
        {userLinks.map(link => (
          <Link
            key={link.href}
            href={link.href}
            className={button({ variant: 'secondary' })}>
            {link.name}
          </Link>
        ))}
        <ConfirmPopover
          label={'Are you sure you want to delete this user?'}
          buttonEl={'Delete'}
          buttonType={'button'}
          popoverSide={'right'}
          confirm={{
            action: deleteUserAction,
            actionPayload: selectedUser.id,
            button: (
              <>
                <Icon.Delete /> Delete
              </>
            ),
          }}
        />
      </HStack>
      {children}
    </Stack>
  );
}
