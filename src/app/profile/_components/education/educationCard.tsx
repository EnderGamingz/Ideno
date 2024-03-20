import { Card } from '@/app/profile/_components/card';
import { Box, HStack, styled } from '@/styling/jsx';
import ConditionalWrapper from '@/app/_components/ConditionalWrapper';
import Icon from '@/app/_components/icon';
import Link from 'next/link';
import { button } from '@/recipes/button';
import { PublicAuthUserModel } from '@/types/user';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import { AuthEducationModel, PublicEducationModel } from '@/types/education';
import AddEducationDialog from '@/app/profile/_components/education/addEducationDialog';
import EditEducationDialog from '@/app/profile/_components/education/editEducationDialog';
import deleteEducationAction from '@/app/profile/_components/education/deleteEducationAction';

export function EducationCard({
  username,
  user,
  data,
}: {
  username: string;
  user?: PublicAuthUserModel;
  data: PublicEducationModel[];
}) {
  const isCurrentUser = username === user?.username;
  if (!isCurrentUser && data.length === 0) return null;
  return (
    <Card>
      <HStack justify={'space-between'}>
        <h2>Education</h2>
        {isCurrentUser && <AddEducationDialog />}
      </HStack>
      <Box mb={2}>
        {data.map((item, i) => (
          <EducationItem
            data={item}
            key={`${item.school}${i}`}
            last={data.length - 1 === i}
          />
        ))}
      </Box>
      {data.length > 0 ? (
        <Link
          className={button({ variant: 'secondary', size: 'small' })}
          href={username + '/educations'}>
          Show All Educations <Icon.Forward />
        </Link>
      ) : (
        <styled.p opacity={0.5}>No educations added</styled.p>
      )}
    </Card>
  );
}

export function EducationItem({
  data,
  last,
}: {
  data: AuthEducationModel;
  last?: boolean;
}) {
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
              <EditEducationDialog data={data} />
              <ConfirmPopover
                label={'Are you sure?'}
                buttonEl={<Icon.Delete />}
                confirm={{
                  action: deleteEducationAction,
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
          {data.school}
        </styled.h3>
        <styled.p>
          {data.degree}
          {data.degree && data.field && ', '} {data.field}
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
          </div>
        </Box>
      </ConditionalWrapper>
    </Box>
  );
}
