import API from '@/lib/api';
import { notFound } from 'next/navigation';
import { Container } from '@/styling/jsx';
import { ContactInformationItem } from '@/app/profile/_components/contactInformation/contactInformationItem';

export default async function Page({
  params: { username },
}: {
  params: { username: string };
}) {
  const contactInformation =
    await API.profile.getContactInformationByUsername(username);
  if (!contactInformation) return notFound();

  return (
    <Container maxWidth={'4xl'} mt={5}>
      {contactInformation.map((item, i) => (
        <ContactInformationItem
          item={item}
          full
          key={`contact-information-${JSON.stringify(item)}-${i}`}
        />
      ))}
    </Container>
  );
}
