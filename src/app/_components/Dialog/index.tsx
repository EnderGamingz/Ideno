import { Dialog } from '@headlessui/react';
import { ReactNode } from 'react';
import { css } from '@/styling/css';

export function DialogTitle({ children }: { children: ReactNode }) {
  return (
    <Dialog.Title
      className={css({
        fontSize: '1.5rem',
        fontWeight: 300,
      })}>
      {children}
    </Dialog.Title>
  );
}

export function DialogDescription({ children }: { children: ReactNode }) {
  return (
    <Dialog.Description
      className={css({
        fontSize: '0.8rem',
        fontWeight: 300,
        mb: 2,
      })}>
      {children}
    </Dialog.Description>
  );
}
