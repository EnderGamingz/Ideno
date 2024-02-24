import API from '@/lib/api';

export default async function Page({ params }: { params: { id: string } }) {
  const selectedUser = await API.auth.admin.user.getById(params.id);
  return (
    <>
      <h2>Showing details for:</h2>
      <h3>&quot; {selectedUser.username} &quot;</h3>
      <pre>{JSON.stringify(selectedUser, null, 2)}</pre>
    </>
  );
}
