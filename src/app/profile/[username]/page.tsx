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

  return (
    <Grid columns={2}>
      <Box bg={'surface'} p={2} shadow={'md'}>
        <HStack justify={'space-between'}>
          <styled.h1 fontSize={'3xl'}>
            {userProfile.profile.firstName || userProfile.profile.lastName
              ? `${userProfile.profile.firstName} ${userProfile.profile.lastName}`
              : username}
          </styled.h1>
          {user?.username === username && (
            <EditProfileDialog profile={userProfile.profile} userId={user.id} />
          )}
        </HStack>
        {userProfile.profile.headline && (
          <styled.p>{userProfile.profile.headline}</styled.p>
        )}
      </Box>

      <h1>Profile</h1>
    </Grid>
  );
}
