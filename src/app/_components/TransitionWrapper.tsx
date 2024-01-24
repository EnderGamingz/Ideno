import { ReactNode } from 'react';
import { Transition } from '@headlessui/react';
import { css } from '@/styling/css';

export default function TransitionWrapper({
  children,
  maxW,
}: {
  children: ReactNode;
  maxW?: string;
}) {
  return (
    <Transition
      className={css({
        maxW,
        pos: 'absolute',
        top: '2.5rem',
        right: 0,
      })}
      enter={css({ transition: 'all 0.2s ease-in-out' })}
      enterFrom={css({ transform: 'scale(0.95)', opacity: 0 })}
      enterTo={css({ transform: 'scale(1)', opacity: 1 })}
      leave={css({ transition: 'all 0.2s ease-in-out' })}
      leaveFrom={css({ transform: 'scale(1)', opacity: 1 })}
      leaveTo={css({ transform: 'scale(0.95)', opacity: 0 })}>
      {children}
    </Transition>
  );
}
