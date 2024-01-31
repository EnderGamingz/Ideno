import {
  AuthCertificationModel,
  PublicCertificationModel,
} from '@/types/certification';
import { Card } from '@/app/profile/_components/card';
import AddCertificationDialog from '@/app/profile/_components/certification/addCertificationDialog';
import { Box, HStack, styled } from '@/styling/jsx';
import ConditionalWrapper from '@/app/_components/ConditionalWrapper';
import Icon from '@/app/_components/icon';
import ExternalLink from '@/app/_components/ExternalLink';
import Link from 'next/link';
import { button } from '@/recipes/button';
import { PublicAuthUserModel } from '@/types/user';
import EditCertificationDialog from '@/app/profile/_components/certification/editCertificationDialog';
import { ConfirmPopover } from '@/app/_components/Dialog/confirmPopover';
import deleteCertificationAction from '@/app/profile/_components/certification/deleteCertificationAction';

export function CertificationCard({
  username,
  user,
  data,
}: {
  username: string;
  user?: PublicAuthUserModel;
  data: PublicCertificationModel[];
}) {
  const isCurrentUser = username === user?.username;
  if (!isCurrentUser && data.length === 0) return null;
  return (
    <Card>
      <HStack justify={'space-between'}>
        <h2>Certifications</h2>
        {isCurrentUser && <AddCertificationDialog />}
      </HStack>
      <Box mb={2}>
        {data.map((item, i) => (
          <CertificationItem
            data={item}
            key={`${item.name + item.organization}${i}`}
            last={data.length - 1 === i}
          />
        ))}
      </Box>
      {data.length > 0 && (
        <Link
          className={button({ variant: 'secondary', size: 'small' })}
          href={username + '/certifications'}>
          Show All Certifications <Icon.Forward />
        </Link>
      )}
    </Card>
  );
}

export function CertificationItem({
  data,
  last,
}: {
  data: AuthCertificationModel;
  last?: boolean;
}) {
  return (
    <Box
      css={{
        borderBottom: !last ? '1px solid' : 'none',
        bct: 'black/90',
        p: 2,
        '& a': {
          display: 'flex',
          alignItems: 'center',
          gap: 1,
          textDecoration: 'underline',
        },
      }}>
      <ConditionalWrapper
        condition={!!data?.id}
        wrapper={c => (
          <HStack justify={'space-between'}>
            <Box>{c}</Box>
            <HStack>
              <EditCertificationDialog data={data} />
              <ConfirmPopover
                label={'Are you sure?'}
                buttonEl={<Icon.Delete size={18} />}
                confirm={{
                  action: deleteCertificationAction,
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
        <ConditionalWrapper
          condition={!!data.credential_url}
          wrapper={c => (
            <ExternalLink href={data.credential_url!}>
              {c}
              <Icon.OpenInNew size={16} />
            </ExternalLink>
          )}>
          <styled.h3 fontSize={'1.1rem'} fontWeight={'semibold'}>
            {data.name}
          </styled.h3>
        </ConditionalWrapper>
        <styled.p>{data.organization}</styled.p>
        <Box ct={'black/50'} fontSize={'0.8rem'}>
          <HStack>
            {data.issue_date && (
              <styled.span>
                Issued{' '}
                {new Date(data.issue_date).toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'short',
                })}
              </styled.span>
            )}
            {data.issue_date && data.expiration_date && '•'}
            {data.expiration_date && (
              <styled.span>
                Expires{' '}
                {new Date(data.expiration_date).toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'short',
                })}
              </styled.span>
            )}
          </HStack>
          {data.credential_id && <span>ID: {data.credential_id}</span>}
        </Box>
      </ConditionalWrapper>
    </Box>
  );
}
