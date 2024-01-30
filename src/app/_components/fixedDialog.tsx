'use client';
import { Fragment, ReactNode } from 'react';
import { Dialog, Transition } from '@headlessui/react';
import { stack } from '@/styling/patterns';
import {
  TransitionBackdrop,
  TransitionChildWrapper,
} from '@/app/_components/TransitionWrapper';
import { css } from '@/styling/css';

export function FixedDialog({
  children,
  open,
  onClose = () => {},
}: {
  closable?: boolean;
  children: ReactNode;
  open: boolean;
  onClose?: () => void;
}) {
  return (
    <Transition appear show={open} as={Fragment}>
      <Dialog
        onClose={onClose}
        className={stack({
          pos: 'fixed',
          inset: 0,
          zIndex: 10,
          justify: 'center',
          alignItems: 'center',
          p: 5,
        })}>
        <TransitionBackdrop />
        <TransitionChildWrapper>
          <Dialog.Panel
            className={css({
              zIndex: 10,
              bg: 'surface',
              rounded: 'md',
              shadow: 'md',
              px: 4,
              py: 3,
              maxW: 'lg',
              w: 'full',
            })}>
            {children}
          </Dialog.Panel>
        </TransitionChildWrapper>
      </Dialog>
    </Transition>
  );
}
