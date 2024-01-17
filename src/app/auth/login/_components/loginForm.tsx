'use client';

import { useFormState } from 'react-dom';
import loginSubmit from '@/app/auth/login/_components/loginAction';

export function LoginForm() {
  const [state, formAction] = useFormState(loginSubmit, {
    errors: undefined,
    success: undefined,
    error: undefined,
  } as any);

  return (
    <form action={formAction}>
      {state.cookie}
      <label htmlFor='username'>Email</label>
      <input type='text' name='username' id='username' />
      <label htmlFor='password'>Password</label>
      <input type='password' name='password' id='password' />
      <button type='submit'>Submit</button>
    </form>
  );
}
