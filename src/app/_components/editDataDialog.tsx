'use client';
import { Fragment, ReactNode, useState } from 'react';
import { useRouter } from 'next/navigation';
import Button from '@/recipes/button';
import { Dialog, Transition } from '@headlessui/react';
import { stack } from '@/styling/patterns';
import {
  TransitionBackdrop,
  TransitionChildWrapper,
} from '@/app/_components/TransitionWrapper';
import { css } from '@/styling/css';
import { DialogDescription, DialogTitle } from '@/app/_components/Dialog';
import { HStack, Stack } from '@/styling/jsx';
import Icon from '@/app/_components/icon';
import Alert from '@/recipes/alert';

export default function EditDataDialog({
  button,
  action,
  title,
  description,
  children,
}: {
  button: ReactNode;
  action: (data: FormData) => any;
  title: string;
  description: string;
  children: ReactNode;
}) {
  const [errors, setErrors] = useState([]);
  const router = useRouter();
  const [isOpen, setIsOpen] = useState(false);
  return (
    <>
      <Button
        variant={'outline'}
        contentType={'icon'}
        onClick={() => setIsOpen(true)}>
        {button}
      </Button>
      <Transition appear show={isOpen} as={Fragment}>
        <Dialog
          onClose={() => {}}
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
              <DialogTitle>{title}</DialogTitle>
              <DialogDescription>{description}</DialogDescription>
              {!!errors?.length && (
                <Alert status={'error'}>
                  {errors.map((error: any) => (
                    <span key={error?.message}>
                      {error?.message} on {error?.path[0]}
                    </span>
                  ))}
                </Alert>
              )}
              <form
                action={async data => {
                  const res = await action(data);
                  if (res?.success) {
                    router.refresh();
                    setIsOpen(false);
                  } else {
                    setErrors(res?.errors);
                  }
                }}>
                <Stack>{children}</Stack>
                <HStack justify={'flex-end'} mt={5}>
                  <Button
                    variant={'secondary'}
                    type={'button'}
                    onClick={() => setIsOpen(false)}>
                    Cancel
                  </Button>
                  <Button type={'submit'}>
                    <Icon.Check /> Submit
                  </Button>
                </HStack>
              </form>
            </Dialog.Panel>
          </TransitionChildWrapper>
        </Dialog>
      </Transition>
    </>
  );
}
