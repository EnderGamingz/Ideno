'use client';
import { ReactNode, useState } from 'react';
import { useRouter } from 'next/navigation';
import Button from '@/recipes/button';
import { DialogDescription, DialogTitle } from '@/app/_components/Dialog/index';
import { HStack, Stack } from '@/styling/jsx';
import Icon from '@/app/_components/icon';
import Alert from '@/recipes/alert';
import { FixedDialog } from '@/app/_components/Dialog/fixedDialog';

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
        contentType={'iconRound'}
        variant={'outline'}
        onClick={() => setIsOpen(true)}>
        {button}
      </Button>
      <FixedDialog open={isOpen}>
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
      </FixedDialog>
    </>
  );
}
