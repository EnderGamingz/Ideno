'use client';

import { useFormState } from 'react-dom';
import loginSubmit from '@/app/auth/login/_components/loginAction';

export function LoginForm() {
  const [state, formAction] = useFormState(loginSubmit, {
    errors: undefined,
    success: undefined,
    error: undefined,
  } as any);

  const isError = (field: string) => {
    if (Array.isArray(state.errors)) {
      return (
        state.errors.filter((x: any) => (x.path[0] || '') === field).length > 0
      );
    }
  };

  return (
    <>
      {state.error ? 'Error' : null}
      {state.success}
      <form action={formAction}>
        <div>
          <label htmlFor='username'>Email</label>
          <input type='text' name='username' id='username' />
          {isError('username') && <span>This field is required</span>}
        </div>
        <div>
          <label htmlFor='password'>Password</label>
          <input type='password' name='password' id='password' />
          {isError('password') && <span>This field is required</span>}
        </div>
        <button type='submit'>Submit</button>
      </form>
    </>
  );
}
