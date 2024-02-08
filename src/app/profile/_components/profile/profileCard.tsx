import { PublicProfileResponse } from '@/types/profile';
import { Card } from '@/app/profile/_components/card';
import { Box, Divider, HStack, styled } from '@/styling/jsx';
import EditProfileDialog from '@/app/profile/_components/profile/editProfileDialog';
import AddContactInformationDialog from '@/app/profile/_components/contactInformation/addContactInformationDialog';

export function ProfileCard({
  username,
  user,
  data,
}: {
  username: string;
  user: any;
  data: PublicProfileResponse;
}) {
  const profile = data.profile;
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
      {!!data.contact_information.length ? (
        <></>
      ) : (
        <AddContactInformationDialog />
      )}
    </Card>
  );
}
