'use client';
import { ReactNode } from 'react';
import { Popover } from '@headlessui/react';
import { css } from '@/styling/css';
import Button, { button } from '@/recipes/button';
import TransitionWrapper from '@/app/_components/TransitionWrapper';
import { Grid, styled } from '@/styling/jsx';
import Icon from '@/app/_components/icon';
import { useRouter } from 'next/navigation';

export type PopoverPosition = 'top' | 'bottom';
export type PopoverSide = 'left' | 'right';

export function ConfirmPopover({
  label,
  buttonEl,
  confirm,
  buttonType = 'icon',
  popoverPosition,
  popoverSide,
}: {
  label: string;
  buttonEl: ReactNode;
  buttonType?: string;
  popoverPosition?: PopoverPosition;
  popoverSide?: PopoverSide;
  confirm: {
    action: (data?: any) => Promise<any>;
    actionPayload?: any;
    button: ReactNode;
    refresh?: boolean;
  };
}) {
  const router = useRouter();

  return (
    <Popover className={css({ pos: 'relative' })}>
      <Popover.Button
        className={css(
          button.raw({
            variant: 'outline',
            contentType: buttonType === 'icon' ? 'icon' : undefined,
          }),
          css.raw({
            w: 'full',
          }),
        )}>
        {buttonEl}
      </Popover.Button>

      <TransitionWrapper side={popoverSide} position={popoverPosition}>
        <Popover.Panel
          className={css({
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
              <styled.p mb={2} fontWeight={'light'} textAlign={'center'}>
                {label}
              </styled.p>
              <Grid columns={2}>
                <Button
                  onClick={async () => {
                    await confirm.action(confirm.actionPayload);
                    if (confirm.refresh) {
                      router.refresh();
                    }
                    close();
                  }}>
                  {confirm.button}
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
