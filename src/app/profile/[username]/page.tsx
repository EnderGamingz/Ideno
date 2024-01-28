import API from '@/lib/api';
import { Grid, Stack } from '@/styling/jsx';
import auth from 'auth';
import { ProfileCard } from '@/app/profile/_components/profile/profileCard';
import { CertificationCard } from '@/app/profile/_components/certification/certificationCard';

export default async function Page({
  params: { username },
}: {
  params: { username: string };
}) {
  const user = await auth();
  const userProfile = await API.profile.getByUsername(username);

  return (
    <Grid columns={{ base: 1, md: 2 }} p={5} gap={5} alignItems={'flex-start'}>
      <ProfileCard data={userProfile} username={username} user={user} />
      <Stack>
        <CertificationCard
          username={username}
          user={user}
          data={userProfile.certification}
        />
      </Stack>
    </Grid>
  );
}
