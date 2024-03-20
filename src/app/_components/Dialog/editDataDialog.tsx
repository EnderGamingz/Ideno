'use client';
import { ReactNode, useState } from 'react';
import Button from '@/recipes/button';
import { DialogDescription, DialogTitle } from '@/app/_components/Dialog/index';
import { FixedDialog } from '@/app/_components/Dialog/fixedDialog';
import { DataUpdateForm } from '@/app/_components/Dialog/dataUpdateForm';

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
  const [isOpen, setIsOpen] = useState(false);
  return (
    <>
      <Button
        contentType={'icon'}
        variant={'outline'}
        onClick={() => setIsOpen(true)}>
        {button}
      </Button>
      <FixedDialog open={isOpen}>
        <DialogTitle>{title}</DialogTitle>
        <DialogDescription>{description}</DialogDescription>
        <DataUpdateForm action={action} setIsOpen={setIsOpen}>
          {children}
        </DataUpdateForm>
      </FixedDialog>
    </>
  );
}
