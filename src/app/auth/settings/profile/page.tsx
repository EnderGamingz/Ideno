import { styled } from '@/styling/jsx';
import { ProfileCard } from '@/app/profile/_components/profile/profileCard';
import auth from 'auth';
import { redirect } from 'next/navigation';
import API from '@/lib/api';

export default async function Page() {
  const user = await auth();
  if (!user) redirect('/auth/login');

  const profile = await API.profile.getByUsername(user.username);
  if (!profile) return 'Profile not found';

  return (
    <>
      <styled.h2 fontSize={'3xl'} mb={4}>
        Profile Settings
      </styled.h2>
      <styled.span my={2}>Public Profile</styled.span>
      <ProfileCard
        username={user.username}
        user={user}
        data={profile}
        isInSettings
      />
    </>
  );
}
