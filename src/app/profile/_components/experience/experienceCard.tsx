import { Card } from '@/app/profile/_components/card';
import { Box, HStack, styled } from '@/styling/jsx';
import ConditionalWrapper from '@/app/_components/ConditionalWrapper';
import Icon from '@/app/_components/icon';
import Link from 'next/link';
import { button } from '@/recipes/button';
import { PublicAuthUserModel } from '@/types/user';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import { AuthExperienceModel, PublicExperienceModel } from '@/types/experience';
import AddExperienceDialog from '@/app/profile/_components/experience/addExperienceDialog';
import deleteExperienceAction from '@/app/profile/_components/experience/deleteExperienceAction';
import EditExperienceDialog from '@/app/profile/_components/experience/editExperienceDialog';
import { formatDistance, formatDistanceToNow } from 'date-fns';

export function ExperienceCard({
  username,
  user,
  data,
}: {
  username: string;
  user?: PublicAuthUserModel;
  data: PublicExperienceModel[];
}) {
  const isCurrentUser = username === user?.username;
  if (!isCurrentUser && data.length === 0) return null;
  return (
    <Card>
      <HStack justify={'space-between'}>
        <h2>Experiences</h2>
        {isCurrentUser && <AddExperienceDialog />}
      </HStack>
      <Box mb={2}>
        {data.map((item, i) => (
          <ExperienceItem
            data={item}
            key={`${item.company}${item.title}${i}`}
            last={data.length - 1 === i}
          />
        ))}
      </Box>
      {data.length > 0 ? (
        <Link
          className={button({ variant: 'secondary', size: 'small' })}
          href={username + '/experiences'}>
          Show All Experiences <Icon.Forward />
        </Link>
      ) : (
        <styled.p opacity={0.5}>No experiences added</styled.p>
      )}
    </Card>
  );
}

export function ExperienceItem({
  data,
  last,
}: {
  data: AuthExperienceModel;
  last?: boolean;
}) {
  const isCurrent = !!data.start_date && !data.end_date;

  const experienceDuration = data.start_date
    ? isCurrent
      ? formatDistanceToNow(data.start_date)
      : formatDistance(data.start_date, data.end_date!)
    : undefined;
  return (
    <Box
      css={{
        borderBottom: !last ? '1px solid' : 'none',
        bct: 'black/90',
        p: 2,
      }}>
      <ConditionalWrapper
        condition={!!data?.id}
        wrapper={c => (
          <HStack justify={'space-between'}>
            <div>{c}</div>
            <HStack>
              <EditExperienceDialog data={data} />
              <ConfirmPopover
                label={'Are you sure?'}
                buttonEl={<Icon.Delete />}
                confirm={{
                  action: deleteExperienceAction,
                  actionPayload: data.id!,
                  refresh: true,
                  button: (
                    <>
                      <Icon.Delete /> Delete
                    </>
                  ),
                }}
              />
            </HStack>
          </HStack>
        )}>
        <styled.h3 fontSize={'1.1rem'} fontWeight={'semibold'}>
          {data.title}
        </styled.h3>
        <styled.p>
          {data.company}
          {data.exp_type && ` • ${data.exp_type}`}
        </styled.p>
        <Box ct={'black/50'} fontSize={'0.8rem'}>
          <div>
            {data.start_date && (
              <styled.span>
                {new Date(data.start_date).toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'short',
                })}
              </styled.span>
            )}
            {data.start_date && data.end_date && ' - '}
            {data.end_date && (
              <styled.span>
                {new Date(data.end_date).toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'short',
                })}
              </styled.span>
            )}
            {data.start_date && (
              <>
                {' • '}
                {experienceDuration}
              </>
            )}
          </div>
        </Box>
      </ConditionalWrapper>
    </Box>
  );
}
