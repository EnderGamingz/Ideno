'use client';
import { useFormStatus } from 'react-dom';
import Button from '@/recipes/button';
import Icon from '@/app/_components/icon';
import React from 'react';

export function SubmitButton({ disabled }: { disabled: boolean }) {
  const { pending } = useFormStatus();
  return (
    <Button disabled={disabled} pending={pending} type='submit'>
      Submit
      <Icon.Login />
    </Button>
  );
}
