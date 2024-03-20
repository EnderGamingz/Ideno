import { PublicProfileResponse } from '@/types/profile';
import { Card } from '@/app/profile/_components/card';
import { Box, Divider, HStack, Stack, styled } from '@/styling/jsx';
import EditProfileDialog from '@/app/profile/_components/profile/editProfileDialog';
import AddContactInformationDialog from '@/app/profile/_components/contactInformation/addContactInformationDialog';
import Link from 'next/link';
import { css } from '@/styling/css';
import { ContactInformationItem } from '@/app/profile/_components/contactInformation/contactInformationItem';

export function ProfileCard({
  username,
  user,
  data,
  isInSettings,
}: {
  username: string;
  user: any;
  data: PublicProfileResponse;
  isInSettings?: boolean;
}) {
  const profile = data.profile;
  const isCurrentUser = username === user?.username;

  const showContactInformation =
    (!isInSettings && isCurrentUser) ||
    (!!data.contact_information.length && !isCurrentUser);
  return (
    <Card sticky>
      <HStack justify={'space-between'}>
        <HStack gap={'0.3rem'}>
          <styled.h1 fontSize={'3xl'} fontWeight={'semibold'}>
            {profile.first_name || profile.last_name
              ? `${profile.first_name} ${profile.last_name}`
              : username}
          </styled.h1>
          {profile.pronouns && (
            <styled.p fontSize={'sm'} ct={'black/50'}>
              ({profile.pronouns})
            </styled.p>
          )}
        </HStack>
        {user?.username === username && <EditProfileDialog profile={profile} />}
      </HStack>
      {profile.headline && <styled.p>{profile.headline}</styled.p>}
      {(profile.country || profile.city) && (
        <styled.p mt={1} fontSize={'sm'} ct={'black/50'}>
          {profile.city}
          {profile.city && profile.country && ' • '}
          {profile.country}
        </styled.p>
      )}
      <Divider my={2} opacity={0.1} />
      {profile.bio && (
        <Box
          css={{
            outline: '1px solid',
            oct: 'black/95',
            py: 1.5,
            px: 2,
            rounded: 'md',
          }}>
          <styled.h3 fontSize={'1.2rem'} fontWeight={'medium'} mb={1}>
            About me
          </styled.h3>
          <styled.p whiteSpace={'pre-wrap'}>{profile.bio}</styled.p>
        </Box>
      )}
      {showContactInformation && (
        <>
          <HStack justify={'space-between'} alignItems={'center'} my={2}>
            <Stack gap={0}>
              <styled.span fontSize={'1.2rem'} fontWeight={'medium'}>
                Contact Information
              </styled.span>
              {isCurrentUser && (
                <Link
                  className={css({
                    fontSize: '0.8rem',
                    color: 'gray.600',
                  })}
                  href={username + '/contact-information'}>
                  Show All
                </Link>
              )}
            </Stack>
            {isCurrentUser && <AddContactInformationDialog />}
          </HStack>
          {data.contact_information.length || isCurrentUser ? (
            <Box
              css={{
                display: 'grid',
                gridTemplateColumns: 'repeat(auto-fill, minmax(250px, 1fr))',
                gap: 2,
              }}>
              {data.contact_information.map((item, i) => (
                <ContactInformationItem
                  item={item}
                  key={`contact-information-${item.value}-${item.type_field}-${i}`}
                />
              ))}
            </Box>
          ) : (
            <styled.p opacity={0.5}>No contact information added</styled.p>
          )}
        </>
      )}
    </Card>
  );
}
