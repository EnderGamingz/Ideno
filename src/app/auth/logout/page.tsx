'use client';
import Button from '@/recipes/button';
import { useMutation } from '@tanstack/react-query';
import LogoutAction from '@/app/auth/logout/_components/logoutAction';

export default function Page() {
  const logout = useMutation({
    mutationFn: () => LogoutAction(),
  });

  return (
    <>
      <Button onClick={() => logout.mutate()}>Logout</Button>
    </>
  );
}
