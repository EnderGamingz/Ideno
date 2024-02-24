'use client';
import { ReactNode, useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import Alert from '@/recipes/alert';
import { HStack, Stack } from '@/styling/jsx';
import Button from '@/recipes/button';
import Icon from '@/app/_components/icon';

export function DataUpdateForm({
  setIsOpen,
  action,
  children,
  cancel = true,
  successMessage,
}: {
  setIsOpen?: (open: boolean) => void;
  action: (data: FormData) => any;
  children: ReactNode;
  cancel?: boolean;
  successMessage?: string;
}) {
  const router = useRouter();
  const [parseErrors, setParseErrors] = useState([]);
  const [error, setError] = useState<string | undefined>(undefined);
  const [success, setSuccess] = useState<boolean>(false);

  useEffect(() => {
    if (success) {
      const timer = setTimeout(() => {
        setSuccess(false);
      }, 3000);
      return () => clearTimeout(timer);
    }
  }, [success]);

  const isError = !!parseErrors?.length || !!error;

  return (
    <>
      {isError && (
        <Alert status={'error'}>
          <Stack>
            {parseErrors?.map((error: any) => (
              <span key={error?.message}>
                {error?.message} on {error?.path[0]}
              </span>
            ))}
            {error && <span>{error}</span>}
          </Stack>
        </Alert>
      )}
      {success && (
        <Alert status={'success'}>
          {successMessage ?? 'Successfully updated'}
        </Alert>
      )}
      <form
        action={async data => {
          const res = await action(data);
          if (res?.success) {
            setIsOpen && setIsOpen(false);
            setSuccess(true);
            setParseErrors([]);
            setError(undefined);
            router.refresh();
          } else {
            setParseErrors(res?.errors);
            setError(res?.error);
          }
        }}>
        <Stack>{children}</Stack>
        <HStack justify={'flex-end'} mt={5}>
          {cancel && (
            <Button
              variant={'secondary'}
              type={'button'}
              onClick={() => setIsOpen && setIsOpen(false)}>
              Cancel
            </Button>
          )}
          <Button type={'submit'}>
            <Icon.Check /> Submit
          </Button>
        </HStack>
      </form>
    </>
  );
}
