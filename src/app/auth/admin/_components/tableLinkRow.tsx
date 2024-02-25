'use client';
import { useRouter } from 'next/navigation';
import { ReactNode } from 'react';

export function TableLinkRow({
  children,
  href,
}: {
  children: ReactNode;
  href: string;
}) {
  const router = useRouter();
  const handleClick = () => {
    router.push(href);
  };
  return <tr onClick={handleClick}>{children}</tr>;
}
