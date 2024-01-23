'use client';

import { useFormState } from 'react-dom';
import registerSubmit from '@/app/auth/register/_components/registerAction';

export function RegisterForm() {
  const [state, formAction] = useFormState(registerSubmit, {
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
          <label htmlFor='username'>Username</label>
          <input type='text' name='username' id='username' />
          {isError('username') && <span>This field is required</span>}
        </div>
        <div>
          <label htmlFor='email'>Email</label>
          <input type='email' name='email' id='email' />
          {isError('email') && <span>This field is required</span>}
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
