import { AuthContactInformationModel } from '@/types/contactInformation';
import { HStack, Stack, styled } from '@/styling/jsx';
import Icon from '@/app/_components/icon';
import ConditionalWrapper from '@/app/_components/ConditionalWrapper';
import Link from 'next/link';

export const contactTypes: ((string | any)[] | string[])[] = [
  [
    'email',
    'Email',
    {
      type: 'email',
      prefix: 'mailto:',
      clickable: true,
    },
  ],
  [
    'phone',
    'Phone',
    {
      type: 'tel',
      prefix: 'tel:',
      clickable: true,
    },
  ],
  [
    'website',
    'Website',
    {
      type: 'text',
      prefix: 'https://',
      visiblePrefix: 'https://',
      clickable: true,
    },
  ],
  ['linkedin', 'LinkedIn'],
  ['github', 'GitHub'],
  ['twitter', 'Twitter'],
  ['facebook', 'Facebook'],
  ['instagram', 'Instagram'],
];

function getContactIcon(type: string | undefined) {
  switch (type) {
    case 'email':
      return Icon.Mail;
    case 'phone':
      return Icon.Phone;
    case 'website':
      return Icon.Web;
    case 'linkedin':
      return Icon.Linkedin;
    case 'github':
      return Icon.Github;
    case 'twitter':
      return Icon.Twitter;
    case 'facebook':
      return Icon.Facebook;
    case 'instagram':
      return Icon.Instagram;
    default:
      return Icon.Error;
  }
}

export function ContactInformationItem({
  item,
}: {
  item: AuthContactInformationModel;
}) {
  const type = contactTypes.find(x => x[0] === item.type_field);
  const contactIcon = getContactIcon(item.type_field)({ size: 22 });
  let isClickable = type?.[2]?.clickable;
  return (
    <ConditionalWrapper
      condition={isClickable}
      wrapper={c => (
        <Link
          target={'_blank'}
          referrerPolicy={'no-referrer'}
          rel={'noopener'}
          href={type?.[2].prefix + item.value}>
          {c}
        </Link>
      )}>
      <Stack
        oct={'black/90'}
        outline={'1px solid'}
        p={1.5}
        rounded={'md'}
        shadow={'sm'}
        transition={'all 0.2s'}
        _hover={
          isClickable && {
            shadow: 'md',
            bgct: 'primary/90',
          }
        }>
        <HStack gap={0}>
          {contactIcon}
          <styled.h3 ml={2} mr={'auto'} fontSize={'1.3rem'}>
            {type?.[1] ?? item.type_field}
          </styled.h3>
          {isClickable && <Icon.OpenInNew size={15} />}
        </HStack>
        <Stack gap={0} px={1}>
          <styled.span fontWeight={'bold'}>
            {type?.[2]?.visiblePrefix}
            {item.value}
          </styled.span>
        </Stack>
      </Stack>
    </ConditionalWrapper>
  );
}
