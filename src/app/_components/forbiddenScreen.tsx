import { Grid, Stack, styled } from '@/styling/jsx';
import Link from 'next/link';
import Icon from '@/app/_components/icon';
import { button } from '@/recipes/button';

export function ForbiddenScreen() {
  return (
    <Grid
      w={'full'}
      h={'full'}
      placeItems={'center'}
      flexGrow={1}
      textAlign={'center'}>
      <Stack>
        <styled.h1 fontSize={'5xl'} fontWeight={'bold'}>
          Access Denied
        </styled.h1>
        <styled.p fontWeight={'light'}>
          You do not have permission to access this page.
        </styled.p>
        <Link href={'/'} className={button()}>
          <Icon.Back /> Return to Home
        </Link>
      </Stack>
    </Grid>
  );
}
