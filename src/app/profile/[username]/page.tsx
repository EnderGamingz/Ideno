import API from '@/lib/api';
import { Box, Grid, HStack, styled } from '@/styling/jsx';
import auth from 'auth';
import EditProfileDialog from '@/app/profile/_components/editProfileDialog';

export default async function Page({
  params: { username },
}: {
  params: { username: string };
}) {
  const user = await auth();
  const userProfile = await API.profile.getByUsername(username);

  let profile = userProfile.profile;
  return (
    <Grid columns={2} p={5} gap={5}>
      <Box bg={'white'} p={2} shadow={'md'} rounded={'lg'}>
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
          {user?.username === username && (
            <EditProfileDialog profile={profile} />
          )}
        </HStack>
        {profile.headline && <styled.p>{profile.headline}</styled.p>}
        {(profile.country || profile.city) && (
          <styled.p mt={1} fontSize={'sm'} ct={'black/50'}>
            {profile.city && `${profile.city} • `}
            {profile.country}
          </styled.p>
        )}
      </Box>

      <h1>Profile</h1>
    </Grid>
  );
}
