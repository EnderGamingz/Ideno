import API from '@/lib/api';
import { notFound } from 'next/navigation';
import { Container } from '@/styling/jsx';
import { ExperienceItem } from '@/app/profile/_components/experience/experienceCard';

export default async function Page({
  params: { username },
}: {
  params: { username: string };
}) {
  const experiences = await API.profile.getExperienceByUsername(username);
  if (!experiences) return notFound();

  return (
    <Container maxWidth={'4xl'} w={'full'} mt={5}>
      {experiences.map((item, i) => (
        <ExperienceItem key={item.company + item.title + i} data={item} />
      ))}
    </Container>
  );
}
