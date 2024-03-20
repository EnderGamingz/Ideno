import API from '@/lib/api';
import { notFound } from 'next/navigation';
import { Container } from '@/styling/jsx';
import { EducationItem } from '@/app/profile/_components/education/educationCard';

export default async function Page({
  params: { username },
}: {
  params: { username: string };
}) {
  const educations = await API.profile.getEducationByUsername(username);
  if (!educations) return notFound();

  return (
    <Container maxWidth={'4xl'} w={'full'} mt={5}>
      {educations.map((item, i) => (
        <EducationItem key={item.school + i} data={item} />
      ))}
    </Container>
  );
}
