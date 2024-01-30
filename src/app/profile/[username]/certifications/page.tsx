import API from '@/lib/api';
import { notFound } from 'next/navigation';
import { Container } from '@/styling/jsx';
import { CertificationItem } from '@/app/profile/_components/certification/certificationCard';

export default async function Page({
  params: { username },
}: {
  params: { username: string };
}) {
  const certifications =
    await API.profile.getCertificationsByUsername(username);
  if (!certifications) return notFound();

  return (
    <Container maxWidth={'4xl'} mt={5}>
      {certifications.map((item, i) => (
        <CertificationItem
          key={item.name + item.organization + i}
          data={item}
        />
      ))}
    </Container>
  );
}
