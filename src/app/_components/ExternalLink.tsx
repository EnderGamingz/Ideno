import Link from 'next/link';

export default function ExternalLink({
  href,
  children,
}: {
  href: string;
  children: any;
}) {
  return (
    <Link
      referrerPolicy={'no-referrer'}
      rel={'noopener noreferrer'}
      href={href}
      target={'_blank'}>
      {children}
    </Link>
  );
}
