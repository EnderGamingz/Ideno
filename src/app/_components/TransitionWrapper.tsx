import { Fragment, ReactNode } from 'react';
import { Transition } from '@headlessui/react';
import { css } from '@/styling/css';
import { Box } from '@/styling/jsx';

export default function TransitionWrapper({
  children,
}: {
  children: ReactNode;
}) {
  return (
    <Transition
      className={css({
        pos: 'absolute',
        top: '2.5rem',
        right: 0,
        zIndex: 10,
      })}
      enter={'transition-200'}
      enterFrom={'opacity-0 scale-95'}
      enterTo={'opacity-100 scale-100'}
      leave={'transition-200'}
      leaveFrom={'opacity-100 scale-100'}
      leaveTo={'opacity-0 scale-95'}>
      {children}
    </Transition>
  );
}

export function TransitionChildWrapper({ children }: { children: ReactNode }) {
  return (
    <Transition.Child
      as={Fragment}
      enter={'transition-200'}
      enterFrom={'opacity-0 scale-95'}
      enterTo={'opacity-100 scale-100'}
      leave={'transition-200'}
      leaveFrom={'opacity-100 scale-100'}
      leaveTo={'opacity-0 scale-95'}>
      {children}
    </Transition.Child>
  );
}

export function TransitionBackdrop() {
  return (
    <Transition.Child
      as={Fragment}
      enter={'transition-200'}
      enterFrom={'opacity-0'}
      enterTo={'opacity-1'}
      leave={'transition-200'}
      leaveFrom={'opacity-1'}
      leaveTo={'opacity-0'}>
      <Box
        css={{
          pos: 'fixed',
          inset: 0,
          bgt: 'black/70',
        }}
      />
    </Transition.Child>
  );
}
