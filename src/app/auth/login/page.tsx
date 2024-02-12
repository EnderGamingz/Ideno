import React from 'react';
import { Stack } from '@/styling/jsx';
import Link from 'next/link';
import { button } from '@/recipes/button';
import Icon from '@/app/_components/icon';

export default function Page() {
  return (
    <Stack mt={4}>
      <Link
        className={button({ variant: 'secondary' })}
        href={'/auth/login/email'}>
        <Icon.Mail /> Login with Email
      </Link>
    </Stack>
  );
}
