import auth from 'auth';

export function NestedAuthenticationTest() {
  const user = auth();
  if (user) {
    return (
      <>
        <pre>{JSON.stringify(user, null, 2)}</pre>
      </>
    );
  }
  return 'Not signed in';
}
