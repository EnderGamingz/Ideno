import { styled } from '@/styling/jsx';
import React from 'react';

export function CredentialInput({
  text,
  type,
  name,
  inputId,
  error,
}: {
  text: string;
  type?: string;
  name: string;
  inputId: string;
  error?: boolean;
}) {
  return (
    <styled.section mt={1}>
      <label htmlFor={inputId}>{text}</label>
      <input type={type ?? 'text'} name={name} id={inputId} required />
      {error && <span>This field is invalid</span>}
    </styled.section>
  );
}
