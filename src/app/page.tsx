import { Stack } from '@/styling/jsx';
import { Hero } from '@/app/hero';

export default async function Page() {
  return (
    <Stack gap={0}>
      <Hero />
    </Stack>
  );
}
