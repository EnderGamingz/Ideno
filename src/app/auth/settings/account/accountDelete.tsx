'use client';

import { Stack, styled } from '@/styling/jsx';
import Alert from '@/recipes/alert';
import Button from '@/recipes/button';
import { useState } from 'react';
import accountDeleteAction from '@/app/auth/settings/account/accountDeleteAction';

export function AccountDelete() {
  const [confirm, setConfirm] = useState(false);
  const handleClick = async () => {
    if (!confirm) setConfirm(true);
    else if (confirm) {
      await accountDeleteAction();
    }
  };
  return (
    <Alert status={'error'}>
      <Stack gap={0} w={'full'}>
        <styled.h4 fontSize={'1.4rem'}>Delete Account</styled.h4>
        <styled.span>This action cannot be undone</styled.span>
        <Button
          ml={'auto'}
          bg={'red.500!'}
          color={'white!'}
          _hover={{ bg: 'red.700!' }}
          onClick={handleClick}>
          {confirm ? 'Confirm Delete' : 'Delete Account'}
        </Button>
      </Stack>
    </Alert>
  );
}
