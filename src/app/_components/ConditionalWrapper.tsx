import { ReactNode } from 'react';
import Link from 'next/link';

export default function ConditionalWrapper({
  condition,
  wrapper,
  children,
  alt,
}: {
  condition?: boolean;
  wrapper: (x: ReactNode) => ReactNode;
  children: ReactNode;
  alt?: (x: ReactNode) => ReactNode;
}) {
  return condition ? wrapper(children) : alt ? alt(children) : children;
}

export function OutsideLinkConditionalWrapper({
  condition,
  children,
  href,
}: {
  condition?: boolean;
  children: ReactNode;
  href: string;
}) {
  return (
    <ConditionalWrapper
      condition={condition}
      wrapper={c => (
        <Link
          target={'_blank'}
          referrerPolicy={'no-referrer'}
          rel={'noopener'}
          href={href}>
          {c}
        </Link>
      )}>
      {children}
    </ConditionalWrapper>
  );
}
