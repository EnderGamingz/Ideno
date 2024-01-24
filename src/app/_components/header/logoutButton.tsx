'use client';

import Button, { button } from '@/recipes/button';
import Icon from '@/app/_components/icon';
import { Popover } from '@headlessui/react';
import { css } from '@/styling/css';
import TransitionWrapper from '@/app/_components/TransitionWrapper';
import { stack } from '@/styling/patterns';
import { Grid, styled } from '@/styling/jsx';
import LogoutAction from '@/app/_components/header/logoutAction';

export default function LogoutButton() {
  return (
    <Popover className={css({ pos: 'relative' })}>
      <Popover.Button
        className={button({ variant: 'outline', contentType: 'icon' })}>
        <Icon.Logout />
      </Popover.Button>
      <TransitionWrapper>
        <Popover.Panel
          className={stack({
            w: 'min(calc(100vw - 2rem), 300px)',
            py: 2,
            px: 3,
            bg: 'surface',
            outline: '1px solid',
            oct: 'black/85',
            rounded: 'md',
            shadow: 'md',
          })}>
          {({ close }) => (
            <>
              <styled.p fontWeight={'light'} textAlign={'center'}>
                Are you sure you want to logout?
              </styled.p>
              <Grid columns={2}>
                <Button
                  onClick={async () => {
                    await LogoutAction();
                  }}>
                  <Icon.Logout />
                  Logout
                </Button>
                <Button variant={'secondary'} onClick={() => close()}>
                  <Icon.Back />
                  Cancel
                </Button>
              </Grid>
            </>
          )}
        </Popover.Panel>
      </TransitionWrapper>
    </Popover>
  );
}
