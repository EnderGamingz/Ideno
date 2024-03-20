import { Box, Divider, Stack } from '@/styling/jsx';
import { Hero } from '@/app/hero';

export default async function Page() {
  return (
    <Stack gap={0}>
      <Hero />
      <Divider thickness={'5px'} />
      <Box bg={'amber.50'} p={5}></Box>
    </Stack>
  );
}
